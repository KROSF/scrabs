use serenity::{
    client::bridge::{gateway::ShardManager, voice::ClientVoiceManager},
    prelude::{Mutex, TypeMapKey},
};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}
