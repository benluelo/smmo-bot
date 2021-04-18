use serenity::framework::standard::macros::group;

#[group]
#[commands(me, world_boss, orphanage)]
pub(crate) struct General;

pub(crate) mod me;
use me::*;

pub(crate) mod world_boss;
use world_boss::*;

pub(crate) mod orphanage;
use orphanage::*;
