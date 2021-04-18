use crate::{Reqwest, SmmoError, DB};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use smmo_api::models::item::Item;
use smmo_api::models::item::ItemId;
use sqlx::{query_as, query_as_unchecked, query_as_with};

use crate::to_embed::ToEmbed;

#[command]
#[aliases(s)]
#[min_args(1)]
pub async fn search(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = ctx
        .data
        .read()
        .await
        .get::<DB>()
        .ok_or(SmmoError::<Item>::InternalError)?
        .clone();

    let search_term = args.single::<String>().unwrap();

    let matching_items = query_as!(
        Item,
        r#"SELECT 
                id              AS "id: ItemId",
                name,
                type            AS "item_type: _",
                description,
                equipable,
                level           AS "level: u32",
                rarity          AS "rarity: _",
                value           AS "value: u32",
                stat1           AS "stat1: _",
                stat1modifier   AS "stat1modifier: u32",
                stat2           AS "stat2: _",
                stat2modifier   AS "stat2modifier: u32",
                stat3           AS "stat3: _",
                stat3modifier   AS "stat3modifier: u32",
                custom_item,
                tradable,
                locked
            FROM
                item
            WHERE
                LOWER(name) LIKE CONCAT('%', $1::TEXT, '%')"#,
        search_term
    )
    .fetch_all(&pool)
    .await
    .map_err(|why| {
        log::error!("{}", why.to_string());
        SmmoError::<Item>::InternalError
    })?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |cm| {
            cm.embed(|e| e.fields(matching_items.iter().take(25).map(|item| item.to_field())))
        })
        .await;
    Ok(())
}
