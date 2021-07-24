use std::{
  collections::HashMap,
  sync::Arc,
};
use tokio::sync::Mutex;
use serenity::prelude::*;
use serenity::client::bridge::gateway::ShardManager;



// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}