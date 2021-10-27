use serenity::client::Context;
use serenity::framework::standard::{macros::command, macros::group, Args, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let one = args.single::<f64>()?;
  let two = args.single::<f64>()?;

  let product = one * two;

  msg.channel_id.say(&ctx.http, product).await?;

  Ok(())
}

#[group]
// Sets a single prefix for this group.
// So one has to call commands in this group
// via `~math` instead of just `~`.
#[prefix = "math"]
#[commands(multiply)]
pub struct Math;
