use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serenity::builder::CreateEmbed;
use std::convert::TryInto;

use crate::smmo::SmmoModel;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WorldBoss {
    pub id: u32,
    pub name: String,
    pub avatar: String,
    pub level: u32,
    pub god: u32,
    pub str: u32,
    pub def: u32,
    pub dex: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    #[serde(with = "ts_seconds")]
    pub enable_time: DateTime<Utc>,
}

impl SmmoModel for WorldBoss {
    const TYPE_NAME: &'static str = "WorldBoss";

    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        let health_percentage = self.current_hp / self.max_hp;
        let ready_in = self.enable_time - Utc::now();
        embed
            .title(&*self.name)
            .description(if self.enable_time >= Utc::now() {
                "Ready to be attacked!".into()
            } else {
                format!("Ready in: {}", ready_in)
            })
            .field(
                "Health",
                format!(
                    "{:░>10} {}/{} ({}%)",
                    "█".repeat((health_percentage * 10).try_into().unwrap()),
                    self.current_hp,
                    self.max_hp,
                    health_percentage * 100
                ),
                true,
            )
            .field(
                "Stats",
                format!("str: {}\ndef: {}\ndex: {}\n", self.str, self.def, self.dex),
                true,
            )
    }

    fn to_field(&self) -> (String, String, bool) {
        (
            self.name.clone(),
            if self.enable_time >= Utc::now() {
                "Ready to be attacked!".into()
            } else {
                format!("Ready in: {}", (self.enable_time - Utc::now()))
            },
            true,
        )
    }
}
