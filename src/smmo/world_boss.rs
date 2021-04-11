use serde::Deserialize;
use serenity::builder::CreateEmbed;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    pub enable_time: u32,
}

const TYPE_NAME: &'static str = "SmmoPlayer";