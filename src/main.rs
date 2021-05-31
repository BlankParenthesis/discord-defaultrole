use std::fs;
use std::collections::HashMap;

use serenity::{async_trait, prelude::*};
use serenity::model::id::{GuildId, RoleId};
use serenity::model::guild::Member;
use serenity::client::bridge::gateway::GatewayIntents;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    token: String,
    roles: HashMap<u64, Vec<RoleId>>,
}

struct Handler {
    config: Config
}

impl Handler {
    fn new(config: Config) -> Self {
        Self {
            config
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(
        &self, 
        context: Context, 
        guild_id: GuildId, 
        mut member: Member,
    ) {
        let maybe_roles = self.config.roles.get(&guild_id.0);

        if let Some(roles) = maybe_roles {
            let result = member.add_roles(context.http, &roles[0..]).await;

            if let Err(error) = result {
                println!(
                    "Failed to add default roles in server {}: {:?}", 
                    guild_id.0, 
                    error
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config_contents = fs::read_to_string("config.json")
        .expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_contents)
        .expect("Unable to parse config file");

    let token = config.token.clone();
        
    let handler = Handler::new(config);

    let mut client = Client::builder(&token)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS)
        .event_handler(handler).await
        .unwrap();
    
    if let Err(cause) = client.start_autosharded().await {
        println!("Client error: {:?}", cause);
    }
}