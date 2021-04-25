use std::fs::File;
use crate::{graph::pie_chart, Reqwest, SmmoError, DB};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use smmo_api::models::item::ItemType;
use smmo_api::models::item::{Item, ItemId, ItemRarity};
use sqlx::{query_as, query_as_unchecked, query_as_with, Executor};

use crate::to_embed::ToEmbed;

#[command]
#[delimiters(" ")]
#[aliases(s)]
// #[min_args(1)]
pub async fn query(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let pool = ctx
    //     .data
    //     .read()
    //     .await
    //     .get::<DB>()
    //     .ok_or(SmmoError::<Item>::InternalError)?
    //     .clone();

    // let clause = args
    //     .iter::<String>()
    //     .map(Result::unwrap)
    //     .collect::<Vec<_>>()
    //     .join(" ");

    // dbg!(&clause);

    // let query = String::from(
    //     r#"SELECT
    //         name,
    //         type            AS "item_type: _",
    //         level::INT4           AS "level: u32",
    //         rarity          AS "rarity: _"
    //     FROM
    //         item
    //     WHERE "#,
    // ) + &*clause;

    // let results: Vec<(String, ItemType, u32, ItemRarity)> =
    //     query_as(&query).fetch_all(&pool).await.map_err(|why| {
    //         log::error!("{}", why);
    //         SmmoError::<Item>::InternalError
    //     })?;

    // let search_term = args.single::<String>().unwrap();

    // let matching_items = query_as!(
    //     Item,
    // r#"SELECT
    //         id              AS "id: ItemId",
    //         name,
    //         type            AS "item_type: _",
    //         description,
    //         equipable,
    //         level           AS "level: u32",
    //         rarity          AS "rarity: _",
    //         value           AS "value: u32",
    //         stat1           AS "stat1: _",
    //         stat1modifier   AS "stat1modifier: u32",
    //         stat2           AS "stat2: _",
    //         stat2modifier   AS "stat2modifier: u32",
    //         stat3           AS "stat3: _",
    //         stat3modifier   AS "stat3modifier: u32",
    //         custom_item,
    //         tradable,
    //         locked
    //     FROM
    //         item
    //     WHERE
    //             LOWER(name) LIKE CONCAT('%', $1::TEXT, '%')"#,
    //     search_term
    // )
    // .fetch_all(&pool)
    // .await
    // .map_err(|why| {
    //     log::error!("{}", why.to_string());
    //     SmmoError::<Item>::InternalError
    // })?;

    let items = [
        (
            696u32,
            ItemRarity::Uncommon.colour_rgb(),
            &*ItemRarity::Uncommon.to_string(),
        ),
        (
            3270,
            ItemRarity::Elite.colour_rgb(),
            &*ItemRarity::Elite.to_string(),
        ),
        (
            1312,
            ItemRarity::Common.colour_rgb(),
            &*ItemRarity::Common.to_string(),
        ),
        (
            382,
            ItemRarity::Exotic.colour_rgb(),
            &*ItemRarity::Exotic.to_string(),
        ),
        (
            1323,
            ItemRarity::Rare.colour_rgb(),
            &*ItemRarity::Rare.to_string(),
        ),
        (
            11136,
            ItemRarity::Celestial.colour_rgb(),
            &*ItemRarity::Celestial.to_string(),
        ),
        (
            3409,
            ItemRarity::Epic.colour_rgb(),
            &*ItemRarity::Epic.to_string(),
        ),
        (
            3905,
            ItemRarity::Legendary.colour_rgb(),
            &*ItemRarity::Legendary.to_string(),
        ),
    ];

    log::warn!("generating chart");
    let file = (&*pie_chart(&items).unwrap(), "chart.bmp");
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |cm| {
            // cm.embed(|e| {
            //     e.fields(
            //         results
            //             .iter()
            //             .take(25)
            //             .map(|(name, item_type, _, _)| (name.clone(), item_type.clone(), true)),
            //     )
            // })
            // cm.add_file((File::create(), file.0, file.1))
            cm
        })
        .await;
    Ok(())
}
