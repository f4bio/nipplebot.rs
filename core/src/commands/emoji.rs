use serenity::client::Context;
use serenity::framework::standard::buckets::RevertBucket;
use serenity::{
  framework::standard::{
    macros::{command, group},
    Args, CommandResult,
  },
  model::channel::Message,
};

#[command]
async fn bird(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let say_content = if args.is_empty() {
    ":bird: can find animals for you.".to_string()
  } else {
    format!(":bird: could not find animal named: `{}`.", args.rest())
  };

  msg.channel_id.say(&ctx.http, say_content).await?;

  Ok(())
}

#[command]
// Adds multiple aliases
#[aliases("kitty", "neko")]
// Make this command use the "emoji" bucket.
#[bucket = "emoji"]
// Allow only administrators to call this:
#[required_permissions("ADMINISTRATOR")]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, ":cat:").await?;

  // We can return one ticket to the bucket undoing the ratelimit.
  Err(RevertBucket.into())
}

#[command]
#[description = "Sends an emoji with a dog."]
#[bucket = "emoji"]
async fn dog(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, ":dog:").await?;

  Ok(())
}

#[group]
// Sets multiple prefixes for a group.
// This requires us to call commands in this group
// via `~emoji` (or `~em`) instead of just `~`.
#[prefixes("emoji", "em")]
// Set a description to appear if a user wants to display a single group
// e.g. via help using the group-name or one of its prefixes.
#[description = "A group with commands providing an emoji as response."]
// Summary only appears when listing multiple groups.
#[summary = "Do emoji fun!"]
// Sets a command that will be executed if only a group-prefix was passed.
#[default_command(bird)]
#[commands(cat, dog)]
pub struct Emoji;
