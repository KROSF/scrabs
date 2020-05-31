use crate::common::{
    framework::ScrabsFramework,
    handler::ScrabsHandler,
    store::{ShardManagerContainer, VoiceManager},
};
use serenity::{prelude::SerenityError, Client};
use std::{collections::HashSet, sync::Arc};

pub struct ScrabsClient(Client);

impl Default for ScrabsClient {
    fn default() -> Self {
        ScrabsClient::new()
    }
}

impl ScrabsClient {
    pub fn new() -> Self {
        let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let mut client =
            Client::new(&token, ScrabsHandler::default()).expect("Err creating client");
        {
            let mut data = client.data.write();
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
            data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        }

        let owners = match client.cache_and_http.http.get_current_application_info() {
            Ok(info) => {
                let mut owners = HashSet::new();
                owners.insert(info.owner.id);

                owners
            }
            Err(why) => panic!("Could not access application info: {:?}", why),
        };

        client.with_framework(ScrabsFramework::with_owners(owners));

        ScrabsClient(client)
    }

    pub fn start(&mut self) -> Result<(), SerenityError> {
        self.start_autosharded()
    }
    pub fn start_autosharded(&mut self) -> Result<(), SerenityError> {
        self.0.start_autosharded()
    }
}
