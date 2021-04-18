use crate::utils::duration_pretty;
use chrono::{Duration, Utc};
use serenity::builder::CreateEmbed;
use smmo_api::models::{
    item::Item,
    orphanage::Orphanage,
    smmo_player::SmmoPlayer,
    world_boss::{WorldBoss, WorldBosses},
};
use std::convert::TryInto;

pub trait ToEmbed {
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed;

    fn to_field(&self) -> (String, String, bool);
}

impl ToEmbed for SmmoPlayer {
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        let safe_mode_response = match (self.safe_mode, self.safe_mode_time) {
            (true, None) => "This player is permanently in safe mode.".to_string(),
            (true, Some(expiry)) => {
                let expires_in = (expiry - Utc::now()) + Duration::days(1);
                let h = expires_in.num_hours();
                let m = expires_in.num_minutes() % 60;
                let s = expires_in.num_seconds() % 60;
                // format!("This player's safe mode expires at {}", expiry.to_rfc3339())
                format!("This player's safe mode expires in {}:{}:{}.", h, m, s)
            }
            (false, _) => "This player is not currently in safe mode.".into(),
        };
        embed
            .title(&*self.name)
            .description(safe_mode_response)
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

impl ToEmbed for WorldBoss {
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        let health_percentage = self.current_hp / self.max_hp;
        let ready_in = self.enable_time - Utc::now();
        embed
            .title(&*self.name)
            .description(if self.enable_time <= Utc::now() {
                "Ready to be attacked!".into()
            } else {
                format!("Ready in: {}", duration_pretty(ready_in))
            })
            .field(
                "Health",
                format!(
                    "**`{:░>10}`**\n{}/{} ({}%)",
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
            if self.enable_time <= Utc::now() {
                "Ready to be attacked!".into()
            } else {
                format!(
                    "Ready in: {}",
                    duration_pretty(self.enable_time - Utc::now())
                )
            },
            true,
        )
    }
}

impl ToEmbed for WorldBosses {
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        embed.fields(self.0.iter().map(|t| t.to_field()))
    }

    fn to_field(&self) -> (String, String, bool) {
        ("test".into(), "test".into(), true)
    }
}

impl ToEmbed for Orphanage {
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        embed
            .title("Orphanage")
            .description(if self.current_amount >= self.max_amount {
                "Goal reached!".into()
            } else {
                format!(
                    "Amount remaining: {}\n({}/{}",
                    self.max_amount - self.current_amount,
                    self.current_amount,
                    self.max_amount
                )
            })
            .color(if self.current_amount >= self.max_amount {
                0x00FF00
            } else {
                0xFF0000
            })
    }

    fn to_field(&self) -> (String, String, bool) {
        (
            "Orphanage".into(),
            if self.current_amount >= self.max_amount {
                "Goal reached!".into()
            } else {
                format!(
                    "Amount remaining: {}\n({}/{}",
                    self.max_amount - self.current_amount,
                    self.current_amount,
                    self.max_amount
                )
            },
            true,
        )
    }
}

impl ToEmbed for Item {
    fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
        embed
            .title(self.name.clone())
            .description(self.description.as_ref().unwrap_or(&"".to_string()))
            .color(self.rarity.colour())
    }

    fn to_field(&self) -> (String, String, bool) {
        (self.name.clone(), self.rarity.to_string(), true)
    }
}
