
use serde::Deserialize;
use serenity::builder::CreateEmbed;

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
    safe_mode_time: Option<u64>,
    background : u32,
    membership : u32,
    guild: SmmoPlayerGuild,
}

impl SmmoModel for SmmoPlayer {
    const TYPE_NAME: &'static str = "SmmoPlayer";
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        embed
            .title(&*self.name)
            .description(if self.safe_mode {
                "You are currently in safe mode."
            } else {
                "You are not currently in safe mode."
            })
            .field(
                "General information",
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
}

#[derive(Debug, Deserialize)]
pub(crate) struct SmmoPlayerGuild {
    id: u32,
    name: String,
}