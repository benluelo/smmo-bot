use const_format::{concatcp, formatcp};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use serenity::builder::CreateEmbed;

use crate::smmo::world_boss::WorldBoss;

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

    fn to_field(&self) -> (String, String, bool);
}

// impl<T: SmmoModel> SmmoModel for Vec<T> {
//     const TYPE_NAME: &'static str = "Vec";

//     fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
//         embed.fields(self.into_iter().map(|t| t.to_field()))
//     }

//     fn to_field(&self) -> (String, String, bool) {
//         ("test".into(), "test".into(), true)
//     }
// }

impl SmmoModel for Vec<WorldBoss> {
    const TYPE_NAME: &'static str = "Vec";

    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        embed.fields(self.iter().map(|t| t.to_field()))
    }

    fn to_field(&self) -> (String, String, bool) {
        ("test".into(), "test".into(), true)
    }
}

mod date_time {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize_option_datefmt<S: Serializer>(
        time: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time.to_rfc3339().serialize(serializer)
    }

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    
    fn datefmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
    
    pub fn deserialize_option_datefmt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(deserialize_with = "datefmt")] DateTime<Utc>);
    
        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|Wrapper(a)| a))
    }
}
