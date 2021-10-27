use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use itconfig;
use serenity::client::bridge::gateway::ShardManager;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::buckets::LimitedFor;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::{
  gateway::Ready,
  id::GuildId,
  interactions::{
    application_command::{
      ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
      ApplicationCommandOptionType,
    },
    Interaction, InteractionResponseType,
  },
  prelude::{Reaction, ResumedEvent},
};
use serenity::prelude::{RwLock, TypeMapKey};
use serenity::{async_trait, Client};
// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use commands::{emoji::*, general::*, math::*, meta::*, owner::*, voice::*};
use common::logging::{get_subscriber, init_subscriber};

use crate::constants::QUEUE_SUBJECT_TASKS;
use crate::hooks::{after, before, delay_action, dispatch_error, normal_message, unknown_command};

mod commands;
mod constants;
mod helpers;
mod hooks;

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommandCounter;

impl TypeMapKey for CommandCounter {
  type Value = Arc<RwLock<HashMap<String, u64>>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MessageCount;

impl TypeMapKey for MessageCount {
  // While you will be using RwLock or Mutex most of the time you want to modify data,
  // sometimes it's not required; like for example, with static data, or if you are using other
  // kinds of atomic operators.
  //
  // Arc should stay, to allow for the data lock to be closed early.
  type Value = Arc<AtomicUsize>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn reaction_add(&self, _ctx: Context, add_reaction: Reaction) {
    // user '356178103996907521'
    //  on '663913727162056717/663913727162056714'
    //  to message '902566972582137866'
    //  reacted with '1️⃣'
    info!(
      "user '{}' on '{}/{}' to message '{}' reacted with '{}'",
      add_reaction.user_id.unwrap(),
      add_reaction.channel_id,
      add_reaction.guild_id.unwrap(),
      add_reaction.message_id,
      add_reaction.emoji
    );
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);

    println!("{} is connected!", ready.user.name);

    let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
      commands
        .create_application_command(|command| command.name("ping").description("A ping command"))
        .create_application_command(|command| {
          command
            .name("id")
            .description("Get a user id")
            .create_option(|option| {
              option
                .name("id")
                .description("The user to lookup")
                .kind(ApplicationCommandOptionType::User)
                .required(true)
            })
        })
        .create_application_command(|command| {
          command
            .name("welcome")
            .description("Welcome a user")
            .create_option(|option| {
              option
                .name("user")
                .description("The user to welcome")
                .kind(ApplicationCommandOptionType::User)
                .required(true)
            })
            .create_option(|option| {
              option
                .name("message")
                .description("The message to send")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
                .add_string_choice(
                  "Welcome to our cool server! Ask me if you need help",
                  "pizza",
                )
                .add_string_choice("Hey, do you want a coffee?", "coffee")
                .add_string_choice(
                  "Welcome to the club, you're now a good person. Well, I hope.",
                  "club",
                )
                .add_string_choice(
                  "I hope that you brought a controller to play together!",
                  "game",
                )
            })
        })
    })
    .await;

    println!(
      "I now have the following global slash commands: {:#?}",
      commands
    );

    let guild_command = GuildId(123456789)
      .create_application_command(&ctx.http, |command| {
        command
          .name("wonderful_command")
          .description("An amazing command")
      })
      .await;

    println!(
      "I created the following guild command: {:#?}",
      guild_command
    );
  }

  async fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
      let content = match command.data.name.as_str() {
        "ping" => "Hey, I'm alive!".to_string(),
        "id" => {
          let options = command
            .data
            .options
            .get(0)
            .expect("Expected user option")
            .resolved
            .as_ref()
            .expect("Expected user object");

          if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = options {
            format!("{}'s id is {}", user.tag(), user.id)
          } else {
            "Please provide a valid user".to_string()
          }
        }
        _ => "not implemented :(".to_string(),
      };

      if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(content))
        })
        .await
      {
        println!("Cannot respond to slash command: {}", why);
      }
    }
  }
}

#[allow(dead_code)]
fn start_subscriber() {
  debug!("starting nats subscriber...");
  tokio::spawn(async move {
    let nc = nats::connect("nats.docker").unwrap();
    let sub = nc.subscribe(&QUEUE_SUBJECT_TASKS).unwrap();
    sub.clone().with_handler(move |msg| {
      debug!("Received {:?}", msg);
      Ok(())
    });

    tokio::signal::ctrl_c()
      .await
      .expect("Could not register ctrl+c handler for nats subscription");
    sub.drain().unwrap();
  });
  debug!("nats subscriber running!");
}

#[tokio::main]
pub async fn main() {
  dotenv::dotenv().ok();
  debug!("environment variables initialized!");

  let subscriber = get_subscriber("core".into(), "debug".into());
  init_subscriber(subscriber);
  debug!("core-logging initialized!");

  let token: String =
    itconfig::get_env("DISCORD_ACCESS_TOKEN").expect("Expected a token in the environment");
  let http = Http::new_with_token(&token);

  // We will fetch your bot's owners and id
  let (owners, _bot_id) = match http.get_current_application_info().await {
    Ok(info) => {
      let mut owners = HashSet::new();
      owners.insert(info.owner.id);

      (owners, info.id)
    }
    Err(why) => panic!("Could not access application info: {:?}", why),
  };

  // Set up config values
  let prefix: &str = itconfig::get_env_or_default("BOT_TRIGGER_PREFIX", "~");

  // Create the framework
  let framework = StandardFramework::new()
    .configure(|c| c.owners(owners).prefix(prefix))
    .group(&GENERAL_GROUP)
    // Set a function to be called prior to each command execution. This
    // provides the context of the command, the message that was received,
    // and the full name of the command that will be called.
    //
    // Avoid using this to determine whether a specific command should be
    // executed. Instead, prefer using the `#[check]` macro which
    // gives you this functionality.
    //
    // **Note**: Async closures are unstable, you may use them in your
    // application if you are fine using nightly Rust.
    // If not, we need to provide the function identifiers to the
    // hook-functions (before, after, normal, ...).
    .before(before)
    // Similar to `before`, except will be called directly _after_
    // command execution.
    .after(after)
    // Set a function that's called whenever an attempted command-call's
    // command could not be found.
    .unrecognised_command(unknown_command)
    // Set a function that's called whenever a message is not a command.
    .normal_message(normal_message)
    // Set a function that's called whenever a command's execution didn't complete for one
    // reason or another. For example, when a user has exceeded a rate-limit or a command
    // can only be performed by the bot owner.
    .on_dispatch_error(dispatch_error)
    // Can't be used more than once per 5 seconds:
    .bucket("emoji", |b| b.delay(5))
    .await
    // Can't be used more than 2 times per 30 seconds, with a 5 second delay applying per channel.
    // Optionally `await_ratelimits` will delay until the command can be executed instead of
    // cancelling the command invocation.
    .bucket("complicated", |b| {
      b.limit(2)
        .time_span(30)
        .delay(5)
        // The target each bucket will apply to.
        .limit_for(LimitedFor::Channel)
        // The maximum amount of command invocations that can be delayed per target.
        // Setting this to 0 (default) will never await/delay commands and cancel the invocation.
        .await_ratelimits(1)
        // A function to call when a rate limit leads to a delay.
        .delay_action(delay_action)
    })
    .await
    // The `#[group]` macro generates `static` instances of the options set for the group.
    // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
    // #name is turned all uppercase
    .group(&GENERAL_GROUP)
    .group(&EMOJI_GROUP)
    .group(&MATH_GROUP)
    .group(&OWNER_GROUP)
    .group(&VOICE_GROUP)
    .help(&MY_HELP);

  let mut client = Client::builder(&token)
    .framework(framework)
    .register_songbird()
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  {
    let mut data = client.data.write().await;

    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    data.insert::<CommandCounter>(Arc::new(RwLock::new(HashMap::default())));
    data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
  }

  let shard_manager = client.shard_manager.clone();
  tokio::spawn(async move {
    tokio::signal::ctrl_c()
      .await
      .expect("Could not register ctrl+c handler for shard manager thread");
    shard_manager.lock().await.shutdown_all().await;
  });

  if let Err(why) = client.start().await {
    error!("Client error: {:?}", why);
  }
}
