use commands::get_commands;
use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use serenity::{
    all::{MessageId, Reaction, ReactionType, UserId},
    async_trait,
    model::{channel::Message, gateway::Ready, voice::VoiceState},
    prelude::*,
};

mod commands;
mod date_helper;
mod json_helper;

struct Handler {
    vote_handler: VoteHandler,
}

struct VoteHandler {
    vote_counts: HashMap<u64, (u64, u64)>, // Message ID -> (User ID, Count)
}

#[async_trait]
impl EventHandler for Handler {
    // Message sent (anywhere) event
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message content matches any command
        let commands = get_commands();
        if let Some(response) = commands.get(&msg.content) {
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
        }
        // Votekick command
        else if msg.content.starts_with("!timeout") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();

            if args.len() < 2 || msg.mentions.is_empty() {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Please add mention user and retry.")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("Votekick: <@{}>\n Please vote '👍'", msg.mentions[0].id),
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            } else {
                let message = msg.id;
                print!(
                    "Message ID: {} , (User ID: {} , 0)",
                    message, msg.mentions[0].id
                );
                // TODO add to vote_handler
                // add message to vote_counts
                /*
                self.vote_handler
                    .vote_counts
                    .insert(message.get(), (msg.mentions[0].id.get(), 0));
                */
            }
        }
    }

    // Bot (self) joins server event
    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{}: {} is connected!",
            date_helper::timestamp_string(),
            ready.user.name
        );
    }

    // Reaction added event
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // bot-log channel id
        println!(
            "{}: User <@{}> added a reaction to message: https://discord.com/channels/{}/{}/{}",
            date_helper::timestamp_string(),
            reaction.user_id.unwrap().get(),
            reaction.guild_id.unwrap().get(),
            reaction.channel_id.get(),
            reaction.message_id.get()
        );

        if reaction.emoji == ReactionType::Unicode("👍".to_string()) {
            print!("👍");
            // TODO implement vote logic
            // check if message is in vote_counts
            // update message vote count
            // check if vote count is enough
            // kick user

            /*ctx.data.write().await.insert::<VoteHandler>(VoteHandler {
                vote_counts: HashMap::new(),
            });
            // Check if this message is part of a votekick
            if let Some((user_to_kick, count)) = handler.vote_counts.get_mut(&reaction.message_id) {
                // Increase the vote count
                *count += 1;
                println!("Vote count for user {} is now {}", user_to_kick, count);
            }*/
        }
    }

    // Joined a voice channel event
    async fn voice_state_update(&self, _ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        // load json to hashmap
        let mut users = json_helper::get_users();

        // Check if a user joins/switches a voice channel
        if old.is_none() && new.channel_id.is_some() {
            // if user has no timestamp -> add timestamp (if users has timestamp -> user switched channel)
            if let Some(user_data) = users.get_mut(&new.user_id.to_string()) {
                if user_data.get("timestamp").unwrap() == &0 {
                    user_data.insert("timestamp".to_string(), date_helper::timestamp());
                }
            } else {
                // -> new user joins channel
                users.insert(
                    new.user_id.to_string(),
                    HashMap::from([
                        ("timestamp".to_string(), date_helper::timestamp()),
                        ("duration".to_string(), 0),
                    ]),
                );
            }

            println!(
                "{}: User <@{}> joined voice channel <#{}>",
                date_helper::timestamp_string(),
                new.user_id,
                new.channel_id.unwrap().get()
            );
        }
        // leaves voice channel
        else if new.channel_id.is_none() {
            // add up duration
            if let Some(user_data) = users.get_mut(&new.user_id.to_string()) {
                let duration = date_helper::elapsed_time(*user_data.get("timestamp").unwrap())
                    + user_data.get("duration").unwrap();
                // remove timestamp
                user_data.insert("timestamp".to_string(), 0);
                user_data.insert("duration".to_string(), duration);
            }
            //
            println!(
                "{}: User <@{}> left voice channel",
                date_helper::timestamp_string(),
                new.user_id
            );
        }
        // load hashmap to json
        json_helper::set_users(users);
    }
    // OTHER EVENTS HERE

    // END OF EVENT LISTENING
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            vote_handler: VoteHandler {
                vote_counts: HashMap::new(),
            },
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!(
            "{}: Client error: {:?}",
            date_helper::timestamp_string(),
            why
        );
    }
}
