use crate::{
    smmo::{world_boss::WorldBoss, SmmoModel},
    Player, Reqwest, SmmoError, SmmoResult, DB,
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use sqlx::query_as;

#[command]
#[aliases(wb)]
#[sub_commands(all, next)]
pub async fn world_boss(ctx: &Context, msg: &Message) -> CommandResult {
    log::warn!("world_boss");
    let all_world_bosses = ctx
        .data
        .read()
        .await
        .get::<Reqwest>()
        .ok_or(0u8)
        .map_err(|_| SmmoResult::<Vec<WorldBoss>>::Err(SmmoError::InternalError))?
        .get_world_bosses()
        .await?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |cm| cm.embed(|e| all_world_bosses.to_embed(e)))
        .await;
    Ok(())
}

#[command]
#[aliases(a)]
pub async fn all(ctx: &Context, msg: &Message) -> CommandResult {
    log::warn!("world_boss all");
    let all_world_bosses = ctx
        .data
        .read()
        .await
        .get::<Reqwest>()
        .ok_or(0u8)
        .map_err(|_| SmmoResult::<Vec<WorldBoss>>::Err(SmmoError::InternalError))?
        .get_world_bosses()
        .await?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |cm| cm.embed(|e| all_world_bosses.to_embed(e)))
        .await;
    Ok(())
}

#[command]
#[aliases(n)]
pub async fn next(ctx: &Context, msg: &Message) -> CommandResult {
    log::warn!("world_boss next");
    let next_wb = ctx
        .data
        .read()
        .await
        .get::<Reqwest>()
        .ok_or(0u8)
        .map_err(|_| SmmoResult::<Vec<WorldBoss>>::Err(SmmoError::InternalError))?
        .get_world_bosses()
        .await?
        .into_iter()
        .reduce(|wb1, wb2| {
            if wb1.enable_time < wb2.enable_time {
                wb1
            } else {
                wb2
            }
        });

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |cm| {
            cm.embed(|e| {
                if let Some(wb) = next_wb {
                    wb.to_embed(e)
                } else {
                    e.title("No world bosses available at this time.")
                }
            })
        })
        .await;
    Ok(())
}
