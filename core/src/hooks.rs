use serenity::client::Context;
use serenity::framework::standard::{macros::hook, CommandResult, DispatchError};
use serenity::model::channel::Message;
use tracing::{debug, error, info};

use crate::CommandCounter;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
  debug!(
    "before: received command '{}' by user '{}'",
    command_name, msg.author.name
  );

  let counter_lock = {
    // While data is a RwLock, it's recommended that you always open the lock as read.
    // This is mainly done to avoid Deadlocks for having a possible writer waiting for multiple
    // readers to close.
    let data_read = ctx.data.read().await;

    // Since the CommandCounter Value is wrapped in an Arc, cloning will not duplicate the
    // data, instead the reference is cloned.
    // We wap every value on in an Arc, as to keep the data lock open for the least time possible,
    // to again, avoid deadlocking it.
    data_read
      .get::<CommandCounter>()
      .expect("Expected CommandCounter in TypeMap.")
      .clone()
  };

  // Just like with client.data in main, we want to keep write locks open the least time
  // possible, so we wrap them on a block so they get automatically closed at the end.
  {
    // The HashMap of CommandCounter is wrapped in an RwLock; since we want to write to it, we will
    // open the lock in write mode.
    let mut counter = counter_lock.write().await;

    // And we write the amount of times the command has been called to it.
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    debug!(
      "before: command '{}' has been run '{}' times",
      command_name, entry
    );
  }

  true // if `before` returns false, command processing doesn't happen.
}

#[hook]
pub async fn after(
  _ctx: &Context,
  _msg: &Message,
  command_name: &str,
  command_result: CommandResult,
) {
  match command_result {
    Ok(()) => info!("after: Processed command '{}'", command_name),
    Err(why) => error!("after: Command '{}' returned error {:?}", command_name, why),
  }
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
  info!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
  debug!("Message is not a command '{}'", msg.content);
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
  // You may want to handle a Discord rate limit if this fails.
  let _ = msg.react(ctx, '⏱').await;
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
  if let DispatchError::Ratelimited(info) = error {
    // We notify them only once.
    if info.is_first_try {
      let _ = msg
        .channel_id
        .say(
          &ctx.http,
          &format!("Try this again in {} seconds.", info.as_secs()),
        )
        .await;
    }
  }
}
