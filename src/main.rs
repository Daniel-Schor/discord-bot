use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use serenity::{
    all::Reaction,
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        //guild::Member,
        id::ChannelId, //{ChannelId, GuildId},
        voice::VoiceState, 
    },
    prelude::*,
};

mod commands;

struct Handler {
    commands: HashMap<String, String>,
}

const BOT_LOG: u64 = 1286760071623217244;

#[async_trait]
impl EventHandler for Handler {
    // Message sent (anywhere) event
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message content matches any command
        if let Some(response) = self.commands.get(&msg.content) {
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Bot (self) joins server event
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // Reaction added event
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // bot-log channel id
        let channel_id = ChannelId::new(BOT_LOG);
        if let Err(why) = channel_id
            .say(
                &ctx.http,
                format!(
                    "User <@{}> added a reaction to message {}",
                    reaction.user_id.unwrap().get(), format!("https://discord.com/channels/{}/{}/{}", reaction.guild_id.unwrap().get(), reaction.channel_id.get(), reaction.message_id.get())
                ),
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }

    // Joined a voice channel event
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        let channel_id = ChannelId::new(1286760071623217244);  // Define the target channel ID

        // Check if a user joins a voice channel
        if old.is_none() && new.channel_id.is_some() {
            if let Err(why) = channel_id.say(
                &ctx.http, 
                format!("User <@{}> joined voice channel <#{}>", new.user_id, new.channel_id.unwrap().get())
            ).await {
                println!("Error sending message: {:?}", why);
            }
        }
        // leaves a voice channel
        else if new.channel_id.is_none() {
            if let Err(why) = channel_id.say(
                &ctx.http, 
                format!("User <@{}> left voice channel", new.user_id)
            ).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    // OTHER EVENTS HERE

    // END OF EVENT LISTENING
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let commands: HashMap<String, String> = commands::get_commands();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_VOICE_STATES;
    //let users: HashMap<String, String> = HashMap::from("337690647404347393", );

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { commands })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
