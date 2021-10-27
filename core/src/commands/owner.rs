use serenity::client::Context;
use serenity::framework::standard::{macros::command, macros::group, Args, CommandResult};
use serenity::model::channel::{Channel, Message};
use tracing::debug;

use crate::ShardManagerContainer;

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
  let data = ctx.data.read().await;

  if let Some(manager) = data.get::<ShardManagerContainer>() {
    msg.reply(ctx, "Shutting down!").await?;
    manager.lock().await.shutdown_all().await;
  } else {
    msg
      .reply(ctx, "There was a problem getting the shard manager")
      .await?;

    return Ok(());
  }

  Ok(())
}

#[command]
async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u64>() {
    if let Err(why) = msg
      .channel_id
      .edit(&ctx.http, |c| c.slow_mode_rate(slow_mode_rate_seconds))
      .await
    {
      debug!("Error setting channel's slow mode rate: {:?}", why);

      format!(
        "Failed to set slow mode to `{}` seconds.",
        slow_mode_rate_seconds
      )
    } else {
      format!(
        "Successfully set slow mode rate to `{}` seconds.",
        slow_mode_rate_seconds
      )
    }
  } else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache).await {
    format!(
      "Current slow mode rate is `{}` seconds.",
      channel.slow_mode_rate.unwrap_or(0)
    )
  } else {
    "Failed to find channel in cache.".to_string()
  };

  msg.channel_id.say(&ctx.http, say_content).await?;

  Ok(())
}

#[group]
#[owners_only]
// Limit all commands to be guild-restricted.
#[only_in(guilds)]
// Summary only appears when listing multiple groups.
#[summary = "Commands for server owners"]
#[commands(slow_mode, quit)]
pub struct Owner;
