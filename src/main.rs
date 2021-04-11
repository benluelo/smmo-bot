#![feature(try_trait)]

use std::{error::Error, fmt::Display, marker::PhantomData};

use std::{fmt::Debug, ops::Try};

use crate::{
    general::*,
    smmo::{world_boss::WorldBoss, SmmoModel},
};
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
use serde::Deserialize;
use serenity::{
    client::{Client, Context},
    framework::{
        standard::{macros::hook, CommandError},
        StandardFramework,
    },
    model::channel::Message,
    prelude::TypeMapKey,
};
use sqlx::PgPool;

use crate::smmo::smmo_player::SmmoPlayer;

pub mod general;
mod models;
mod smmo;

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
const LOG_LEVEL: LevelFilter = LevelFilter::Warn;
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
                .build(LevelFilter::Warn),
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

pub struct SmmoClient {
    api_key: String,
    inner: reqwest::Client,
}

impl SmmoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            inner: reqwest::Client::new(),
        }
    }

    pub(crate) async fn get_player_by_smmo_id(&self, smmo_id: String) -> SmmoResult<SmmoPlayer> {
        let url = format!("https://api.simple-mmo.com/v1/player/info/{}", smmo_id);
        match self
            .inner
            .post(&*url)
            .query(&[("api_key", &*self.api_key)])
            .send()
            .await
        {
            Ok(res) => {
                // dbg!(&res.text().await);
                // SmmoResult::InternalError
                let req_url = res.url().to_string();
                match res.text().await {
                    Ok(text) => match serde_json::from_str::<SmmoResult<SmmoPlayer>>(&*text) {
                        Ok(json) => json,
                        Err(why) => SmmoResult::Err(SmmoError::JsonDecodeError(text, req_url)),
                    },
                    Err(why) => {
                        log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                        SmmoResult::Err(SmmoError::ApiError(why))
                    }
                }
            }
            Err(why) => {
                log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                SmmoResult::Err(SmmoError::InternalError)
            }
        }
    }

    pub(crate) async fn get_world_bosses(&self) -> SmmoResult<Vec<WorldBoss>> {
        let url = "https://api.simple-mmo.com/v1/worldboss/all";
        match self
            .inner
            .post(&*url)
            .query(&[("api_key", &*self.api_key)])
            .send()
            .await
        {
            Ok(res) => {
                // dbg!(&res.text().await);
                // SmmoResult::InternalError
                let req_url = res.url().to_string();
                match res.text().await {
                    Ok(text) => match serde_json::from_str::<SmmoResult<Vec<WorldBoss>>>(&*text) {
                        Ok(json) => json,
                        Err(why) => SmmoResult::Err(SmmoError::JsonDecodeError(text, req_url)),
                    },
                    Err(why) => {
                        log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                        SmmoResult::Err(SmmoError::ApiError(why))
                    }
                }
            }
            Err(why) => {
                log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                SmmoResult::Err(SmmoError::InternalError)
            }
        }
    }
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, res: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(err) = res {
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SmmoResult<T: SmmoModel> {
    Ok(T),
    Err(SmmoError<T>),
}

impl<T: SmmoModel> Try for SmmoResult<T> {
    type Ok = T;

    type Error = SmmoError<T>;

    fn into_result(self) -> Result<<SmmoResult<T> as Try>::Ok, Self::Error> {
        match self {
            SmmoResult::Ok(ok) => Ok(ok),
            SmmoResult::Err(err) => Err(err),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        SmmoResult::Err(v)
    }

    fn from_ok(v: <SmmoResult<T> as Try>::Ok) -> Self {
        SmmoResult::Ok(v)
    }
}

impl<T: SmmoModel> Display for SmmoResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            SmmoResult::Ok(t) => t.to_field().0,
            SmmoResult::Err(err) => err.to_string(),
        };
        f.write_str(&string)
    }
}

impl<T: SmmoModel + Debug> Error for SmmoResult<T> {}

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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SmmoError<T: SmmoModel> {
    /// Error from the api; means the api_key is not valid.
    Unauthenticated,
    /// Something went wrong internally, check the logs.
    #[serde(skip)]
    InternalError,
    /// Unable to deserialize the api response; most likely means that the response structure changed.
    #[serde(skip)]
    JsonDecodeError(String, String),
    /// Something went wrong when fetching from the smmo api.
    #[serde(skip)]
    ApiError(reqwest::Error),
    /// Used to appease the typechecker. should never be constructed.
    #[serde(skip)]
    PhantomData(PhantomData<T>),
}

impl<T: SmmoModel> Display for SmmoError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                SmmoError::Unauthenticated => {
                    "Authentication error with the SMMO api. Check the api key.".into()
                }
                SmmoError::InternalError => "Something went wrong! Check the logs!".into(),

                SmmoError::JsonDecodeError(original, url) => format!(
                    r#"JSON decode error.
URL: `{}`
JSON recieved from the api: ```{}```
Expected type: {}
"#,
                    url,
                    original,
                    T::TYPE_NAME
                ),

                SmmoError::ApiError(error) => format!("Error with the SMMO api: ```{}```", error),

                SmmoError::PhantomData(_) => {
                    unreachable!("PhantomData variant should never be constructed.")
                }
            },
        ))
    }
}

impl<T: SmmoModel + std::fmt::Debug> std::error::Error for SmmoError<T> {}
