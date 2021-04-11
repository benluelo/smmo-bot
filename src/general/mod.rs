use serenity::framework::standard::macros::group;

#[group]
#[commands(me)]
pub(crate) struct General;

pub(crate) mod me;
use me::*;
