use dotenv::dotenv;
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use serenity::{
    client::{Client, Context},
    framework::{
        standard::{macros::hook, CommandError},
        StandardFramework,
    },
    model::channel::Message,
    prelude::TypeMapKey,
};
use smmo_api::client::{SmmoClient, SmmoError};
use sqlx::PgPool;

use crate::{general::*, item::*};
pub mod general;
mod graph;
pub mod item;
mod models;
mod to_embed;
mod utils;

struct Player {
    discord_id: String,
    smmo_id: String,
}

struct DB;

impl TypeMapKey for DB {
    type Value = PgPool;
}

struct Reqwest;

impl TypeMapKey for Reqwest {
    type Value = SmmoClient;
}

const PATTERN: &str = "[{d(%v %T)(utc)} {target} {highlight({level})}: {file}:{line}] {message}{n}";
const LOG_LEVEL: LevelFilter = LevelFilter::Info;
const FILE_PATH: &str = "./tmp/out.log";

#[tokio::main]
async fn main() {
    dotenv().ok();

    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(build_log_config()).unwrap();

    // let reqwest_client = build_reqwest_client();
    let pool = build_postgres_pool().await;
    let framework = build_command_framework();

    if let Ok(mut client) =
        Client::builder(dotenv::var("BOT_TOKEN").expect("BOT_TOKEN environment variable not set."))
            .framework(framework)
            .await
    {
        {
            let mut data = client.data.write().await;
            data.insert::<DB>(pool);
            data.insert::<Reqwest>(SmmoClient::new(
                dotenv::var("SMMO_API_TOKEN")
                    .expect("SMMO_API_TOKEN environment variable not set."),
            ))
        };
        if let Err(why) = client.start().await {
            println!("Unable to start client: {:?}", why);
            std::process::exit(-1);
        };
    };
}

fn build_log_config() -> Config {
    Config::builder()
        .appender(
            Appender::builder().build(
                "logfile",
                Box::new(
                    RollingFileAppender::builder()
                        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
                        .encoder(Box::new(PatternEncoder::new(PATTERN)))
                        .build(
                            FILE_PATH,
                            Box::new(CompoundPolicy::new(
                                Box::new(SizeTrigger::new(5 * 1024)),
                                Box::new(FixedWindowRoller::builder().build("{}.log", 3).unwrap()),
                            )),
                        )
                        .unwrap(),
                ),
            ),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LOG_LEVEL)))
                .build(
                    "stderr",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(PatternEncoder::new(PATTERN)))
                            .target(Target::Stderr)
                            .build(),
                    ),
                ),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LOG_LEVEL),
        )
        .unwrap()
}

fn build_command_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix("$");
            c.with_whitespace(true)
        })
        .after(after_hook)
        .group(&GENERAL_GROUP)
        .group(&ITEM_GROUP)
}

async fn build_postgres_pool() -> sqlx::Pool<sqlx::Postgres> {
    PgPool::connect(
        &*dotenv::var("DATABASE_URL").expect("DATABASE_URL environment variable not set."),
    )
    .await
    .map_err(|why| {
        println!("Unable to start client: {:?}", why);
        std::process::exit(-1);
    })
    .unwrap()
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, res: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(err) = res {
        log::error!(target: "command_errors", "{}, {}", cmd_name, err);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, |cm| {
                cm.embed(|e| {
                    e.title("Uh-oh!")
                        .description(&*format!("Something went wrong!\n\nError: {}", err))
                })
            })
            .await;
    }
}

// impl<T> SmmoResult<T> {
//     pub fn map_err_to_msg(
//         self,
//         ctx: &Context,
//         channel_id: ChannelId,
//     ) -> Result<T, impl Future<Output = Result<Message, serenity::Error>>> {
//         if let Self::Err(why) = self {
//             match why {
//                 SmmoError::Unauthenticated => {
//                     Err(channel_id.send_message(&ctx.http, |cm| {
//                         cm.embed(|e| {
//                             e.title("Uh-oh!")
//                                 .color(0xFF0000)
//                                 .description("Something went wrong! Try again in a moment.")
//                         })
//                     }))
//                 }
//                 SmmoError::InternalError => {}
//                 SmmoError::JsonDecodeError => {}
//             }
//         }
//     }
// }
