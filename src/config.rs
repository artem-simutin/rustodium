use serenity::prelude::GatewayIntents;
use std::env;
pub struct Config {
    pub prefix: String,
    pub intents: GatewayIntents,
    pub token: String,
}

impl Config {
    pub fn new() -> Config {
        let token = env::var("BOT_TOKEN").expect("Token must be specified to use this application");
        Config {
            prefix: "~".to_string(),
            token,
            intents: GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILDS,
        }
    }
}
