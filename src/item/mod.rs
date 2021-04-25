use serenity::framework::standard::macros::group;

#[group]
#[commands(search, query)]
pub(crate) struct Item;

pub(crate) mod search;
use search::*;

pub(crate) mod query;
use query::*;
