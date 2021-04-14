use crate::{smmo::SmmoModel, Player, Reqwest, SmmoResult, DB};
use crate::{SmmoError, SmmoPlayer};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use sqlx::query_as;

#[command]
pub async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let rw_lock_read_guard = ctx.data.read().await;
    let pool = rw_lock_read_guard
        .get::<DB>()
        .ok_or(0u8)
        .map_err(|_| SmmoResult::<SmmoPlayer>::Err(SmmoError::InternalError))?;

    if let Some(player) = query_as!(
        Player,
        "SELECT * from player where discord_id = $1",
        &*msg.author.id.to_string()
    )
    .fetch_optional(pool)
    .await?
    {
        if let Some(client) = rw_lock_read_guard.get::<Reqwest>() {
            let smmo_player = client.get_player_by_smmo_id(player.smmo_id).await?;

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
