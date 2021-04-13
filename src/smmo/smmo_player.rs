use chrono::{serde::ts_seconds_option, DateTime, Duration, Utc};
use serde::Deserialize;
use serenity::{builder::CreateEmbed, utils::Colour};

use crate::smmo::date_time::*;
use crate::smmo::SmmoModel;

#[derive(Debug, Deserialize)]
pub(crate) struct SmmoPlayer {
    id: u32,
    name: String,
    level: u32,
    motto: String,
    profile_number: String,
    exp: u32,
    gold: u32,
    steps: u32,
    npc_kills: u32,
    user_kills: u32,
    quests_complete: u32,
    dex: u32,
    def: u32,
    str: u32,
    bonus_dex: u32,
    bonus_def: u32,
    bonus_str: u32,
    hp: u32,
    max_hp: u32,
    #[serde(rename = "safeMode")]
    #[serde(deserialize_with = "super::bool_from_int")]
    safe_mode: bool,
    #[serde(rename = "safeModeTime")]
    #[serde(
        deserialize_with = "deserialize_option_datefmt",
        serialize_with = "serialize_option_datefmt"
    )]
    safe_mode_time: Option<DateTime<Utc>>,
    background: u32,
    membership: u32,
    guild: Option<SmmoPlayerGuild>,
}

impl SmmoModel for SmmoPlayer {
    const TYPE_NAME: &'static str = "SmmoPlayer";

    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        let safe_mode_response = match (self.safe_mode, self.safe_mode_time) {
            (true, None) => "This player is permanently in safe mode.".to_string(),
            (true, Some(expiry)) => {
                format!("This player's safe mod expires in {}.", Utc::now() - expiry)
            }
            (false, _) => "This player is not currently in safe mode.".into(),
        };
        embed
            .title(&*self.name)
            .description(safe_mode_response)
            .colour(Colour::from_rgb(167, 180, 255))
            .field(
                "General",
                format!("Level: {}\nGold: {}", self.level, self.gold),
                true,
            )
            .field(
                "Stats",
                format!(
                    "str: {} (+ {} bonus)\ndef: {} (+ {} bonus)\ndex: {} (+ {} bonus)\n",
                    self.str, self.bonus_str, self.def, self.bonus_def, self.dex, self.bonus_dex
                ),
                true,
            )
    }

    fn to_field(&self) -> (String, String, bool) {
        (
            self.name.clone(),
            format!("Level: {}\nGold: {}", self.level, self.gold),
            true,
        )
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct SmmoPlayerGuild {
    id: u32,
    name: String,
}
