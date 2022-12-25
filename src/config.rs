use std::env;

use serenity::prelude::GatewayIntents;

pub struct Config {
    pub token: String,
    pub prefix: String,
    pub intents: GatewayIntents,
}

impl Config {
    pub fn new() -> Config {
        Config {
            token: env::var("DISCORD_TOKEN").expect("Discrod token must be in environment"),
            prefix: ">>".to_string(),
            intents: GatewayIntents::non_privileged()
                | GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::DIRECT_MESSAGES,
        }
    }
}
