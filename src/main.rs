mod config;
use config::Config;

use rustodium::load_env_from_file;

// Serenity framework imports
use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        self.
        println!("new message: {:#?}", msg);
        if let Err(why) = msg.reply(ctx, "Hey, it's me!").await {
            println!("Something went wrong replying on message: {}", why);
        };
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Bot has been started successfully. Enjoy 😊");
    }
}

#[tokio::main]
async fn main() {
    load_env_from_file();

    let config = Config::new();

    // Create framework
    let framework = StandardFramework::new().configure(|c| c.prefix(&config.prefix));

    // Create a client
    let mut client = Client::builder(&config.token, config.intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Something went wrong creating the client");

    // Start the client
    if let Err(why) = client.start().await {
        println!(
            "Something went wrong starting the client. Reason: {:?}",
            why
        )
    }
}
