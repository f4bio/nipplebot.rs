use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::framework::standard::{macros::command, macros::group, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

use crate::constants::QUEUE_SUBJECT_TASKS;

#[derive(Serialize, Deserialize, Debug)]
struct BotAction {
  name: &'static str,
  channel_id: ChannelId,
  args: Vec<&'static str>,
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  // TODO:
  let bot_action = BotAction {
    name: "say",
    args: vec!["Pong!"],
    channel_id: msg.channel_id,
  };
  let nc = nats::connect("nats.docker").unwrap();
  nc.publish(
    &QUEUE_SUBJECT_TASKS,
    serde_json::to_vec(&bot_action).unwrap(),
  )
  .unwrap();

  msg.channel_id.say(&ctx.http, "Pong!").await.unwrap();
  Ok(())
}

#[group]
#[commands(ping)]
pub struct General;
