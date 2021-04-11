use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use serenity::builder::CreateEmbed;

pub(crate) mod smmo_player;
pub(crate) mod world_boss;

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub trait SmmoModel {
    const TYPE_NAME: &'static str;
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed;
}
