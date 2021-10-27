use crate::commands::check_msg;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, macros::group, Args, CommandResult};
use serenity::model::channel::{ChannelType, GuildChannel, Message};
use serenity::model::guild::Member;
use serenity::model::user::User;
use tracing::debug;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
  let guild = msg.guild(&ctx.cache).await.unwrap();
  let guild_id = guild.id;

  let channel_id = guild
    .voice_states
    .get(&msg.author.id)
    .and_then(|voice_state| voice_state.channel_id);

  let connect_to = match channel_id {
    Some(channel) => channel,
    None => {
      check_msg(msg.reply(ctx, "Not in a voice channel").await);

      return Ok(());
    }
  };

  let manager = songbird::get(ctx)
    .await
    .expect("Songbird Voice client placed in at initialisation.")
    .clone();

  let _handler = manager.join(guild_id, connect_to).await;

  Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
  let guild = msg.guild(&ctx.cache).await.unwrap();
  let guild_id = guild.id;

  let manager = songbird::get(ctx)
    .await
    .expect("Songbird Voice client placed in at initialisation.")
    .clone();
  let has_handler = manager.get(guild_id).is_some();

  if has_handler {
    if let Err(e) = manager.remove(guild_id).await {
      check_msg(
        msg
          .channel_id
          .say(&ctx.http, format!("Failed: {:?}", e))
          .await,
      );
    }

    check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
  } else {
    check_msg(msg.reply(ctx, "Not in a voice channel").await);
  }

  Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let url = match args.single::<String>() {
    Ok(url) => url,
    Err(_) => {
      check_msg(
        msg
          .channel_id
          .say(&ctx.http, "Must provide a URL to a video or audio")
          .await,
      );
      return Ok(());
    }
  };

  if !url.starts_with("http") {
    check_msg(
      msg
        .channel_id
        .say(&ctx.http, "Must provide a valid URL")
        .await,
    );
    return Ok(());
  }

  let guild = msg.guild(&ctx.cache).await.unwrap();
  let guild_id = guild.id;

  let manager = songbird::get(ctx)
    .await
    .expect("Songbird Voice client placed in at initialisation.")
    .clone();

  if let Some(handler_lock) = manager.get(guild_id) {
    let mut handler = handler_lock.lock().await;

    let source = match songbird::ytdl(&url).await {
      Ok(source) => source,
      Err(why) => {
        println!("Err starting source: {:?}", why);

        check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

        return Ok(());
      }
    };

    handler.play_source(source);

    check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
  } else {
    check_msg(
      msg
        .channel_id
        .say(&ctx.http, "Not in a voice channel to play in")
        .await,
    );
  }

  Ok(())
}

#[command]
async fn come(ctx: &Context, msg: &Message) -> CommandResult {
  // let channel: ChannelId = msg.channel_id;
  let mut author: User = msg.author.clone();
  author.refresh(&ctx.http).await.unwrap();

  let guild_channels: Vec<GuildChannel> = ctx
    .http
    .get_channels(*msg.guild_id.unwrap().as_u64())
    .await
    .unwrap();
  for gc in guild_channels.iter() {
    let chan: GuildChannel = gc.clone();
    if chan.kind == ChannelType::Voice {
      debug!("channel '{}' is a voice channel, joining...", chan.name);
      let chan_members: Vec<Member> = chan.members(&ctx.cache).await.unwrap();
      debug!("chan_members: '{:?}'", chan_members);
    } else {
      debug!("channel '{}' is not a voice channel, skipping", chan.name);
    }
  }

  Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let sentence = args.single::<String>().unwrap();

  // let channel: ChannelId = msg.channel_id;
  let mut author: User = msg.author.clone();
  author.refresh(&ctx.http).await.unwrap();

  let guild_channels: Vec<GuildChannel> = ctx
    .http
    .get_channels(*msg.guild_id.unwrap().as_u64())
    .await
    .unwrap();

  debug!("message's guild's channels: {:?}", guild_channels);

  msg
    .channel_id
    .say(&ctx.http, format!("saying: '{}'", sentence))
    .await
    .unwrap();

  Ok(())
}

#[group]
// Set a description to appear if a user wants to display a single group
// e.g. via help using the group-name or one of its prefixes.
#[description = "A group with commands providing an all available voice features."]
// Summary only appears when listing multiple groups.
#[summary = "Do voice stuff!"]
// Sets a command that will be executed if only a group-prefix was passed.
#[default_command(say)]
#[commands(come, say, play, leave, join)]
pub struct Voice;
