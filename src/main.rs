use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    commands: HashMap<String, String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message content matches any command
        if let Some(response) = self.commands.get(&msg.content) {
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Read the commands from the commands.json file
    let mut file = File::open("commands.json").expect("Could not open commands.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read commands.json");

    let commands: HashMap<String, String> =
        serde_json::from_str(&contents).expect("Could not parse commands.json");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { commands })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
