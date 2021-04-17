use crate::to_embed::ToEmbed;
use crate::Player;
use crate::Reqwest;
use crate::DB;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use smmo_api::{
    client::{SmmoError},
    models::smmo_player::SmmoPlayer,
};
use sqlx::query_as;

#[command]
pub async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = ctx
        // .clone()
        .data
        .read()
        .await
        .get::<DB>()
        .ok_or(0u8)
        .map_err(|_| SmmoError::<SmmoPlayer>::InternalError)?
        .clone();

    if let Some(player) = query_as!(
        Player,
        "SELECT * from player where discord_id = $1",
        &*msg.author.id.to_string()
    )
    .fetch_optional(&pool)
    .await?
    {
        if let Some(client) = ctx.data.read().await.get::<Reqwest>() {
            let smmo_player = client.get_player_by_smmo_id(player.smmo_id).await?;

            let _ = msg
                .channel_id
                .send_message(&ctx.http, |cm| cm.embed(|e| smmo_player.to_embed(e)))
                .await;
        }
    } else {
        let _ = msg.channel_id
                .send_message(&ctx.http, |cm| {
                    cm.embed(|e| e.title("Who are you?").description("I don't know who you are! Send `$myid <your smmo id>` to identify yourself."))
                })
                .await;
    }
    Ok(())
}
