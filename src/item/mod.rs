use serenity::framework::standard::macros::group;

#[group]
#[commands(search)]
pub(crate) struct Item;

pub(crate) mod search;
use search::*;
