use crate::{smmo::SmmoModel, Player, Reqwest, SmmoResult, DB};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use sqlx::query_as;

#[command]
pub async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let player = match ctx.data.read().await.get::<DB>() {
        Some(pool) => {
            match query_as!(
                Player,
                "SELECT * from player where discord_id = $1",
                &*msg.author.id.to_string()
            )
            .fetch_optional(pool)
            .await
            {
                Ok(player) => player,
                Err(why) => {
                    let _ = msg
                        .channel_id
                        .send_message(&ctx.http, |cm| {
                            cm.embed(|e| {
                                e.title("Uh-oh!").color(0xFF0000).description(format!(
                                    "Something went wrong!\n\nError: ```{}```",
                                    why
                                ))
                            })
                        })
                        .await;
                    return Ok(());
                }
            }
        }
        None => {
            let _ = msg
                .channel_id
                .send_message(&ctx.http, |cm| {
                    cm.embed(|e| {
                        e.title("Uh-oh!")
                            .color(0xFF0000)
                            .description("Unable to obtain a lock  on the database.")
                    })
                })
                .await;
            return Ok(());
        }
    };

    if let Some(player) = player {
        if let Some(client) = ctx.data.read().await.get::<Reqwest>() {
            let smmo_player = match client.get_player_by_id(player.smmo_id).await {
                SmmoResult::Ok(res) => res,
                _ => {
                    let _ = msg
                        .channel_id
                        .send_message(&ctx.http, |cm| {
                            cm.embed(|e| {
                                e.title("Uh-oh!")
                                    .color(0xFF0000)
                                    .description("Something went wrong! Try again in a moment.")
                            })
                        })
                        .await;
                    return Ok(());
                }
            };
            log::warn!("lkiadsjfj");

            let _ = msg
                .channel_id
                .send_message(&ctx.http, |cm| cm.embed(|e| smmo_player.to_embed(e)))
                .await;
        }
    } else {
        let _ = msg
            .channel_id
            .send_message(&ctx.http, |cm| {
                cm.embed(|e| e.title("Who are you?").description("I don't know who you are! Send `$myid <your smmo id>` to identify yourself."))
            })
            .await;
    }
    Ok(())
}
