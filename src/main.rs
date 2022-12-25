// Commands
mod commands;
use commands::ping::*;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler};
use serenity::{async_trait, Client};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // This function will be called when the bot is ready to start working
    async fn ready(&self, _: Context, _: Ready) {
        println!("Bot is successfully started 😀!");
    }
}

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    rustodium::load_env_from_file();
    let config = rustodium::config::Config::new();

    // Create framework that will handle and dispatch commands
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix))
        .group(&GENERAL_GROUP);

    // Create a new instance of the Serenity client
    let mut client = Client::builder(&config.token, config.intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Cannot create client");

    // Start the client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
