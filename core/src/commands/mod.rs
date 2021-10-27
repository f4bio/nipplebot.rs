use serenity::model::channel::Message;
use serenity::Result as SerenityResult;
use tracing::error;

pub mod emoji;
pub mod general;
pub mod math;
pub mod meta;
pub mod owner;
pub mod voice;

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
  if let Err(why) = result {
    error!("Error sending message: {:?}", why);
  }
}
