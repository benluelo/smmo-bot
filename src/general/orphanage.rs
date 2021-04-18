use crate::{Reqwest, SmmoError};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use smmo_api::models::orphanage::Orphanage;

use crate::to_embed::ToEmbed;

#[command]
#[aliases(o, bonus, goal)]
pub async fn orphanage(ctx: &Context, msg: &Message) -> CommandResult {
    log::warn!("world_boss");
    let orphanage = ctx
        .data
        .read()
        .await
        .get::<Reqwest>()
        .ok_or(SmmoError::<Orphanage>::InternalError)?
        .get_orphanage()
        .await?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |cm| cm.embed(|e| orphanage.to_embed(e)))
        .await;
    Ok(())
}
