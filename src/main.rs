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

impl TypeMapKey for VoteHandler {
    type Value = VoteHandler;
}

async fn votekick_init(ctx: &Context, msg: &Message) {
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

    if let Ok(votekick_message) = msg
        .channel_id
        .say(
            &ctx.http,
            format!(
                "Timeout: <@{}> for 1 min.\n Please vote with a ✅ on this message.",
                msg.mentions[0].id
            ),
        )
        .await
    {
        let msg_id = votekick_message.id;

        // Begin accessing the data map
        let mut data = ctx.data.write().await;
        if let Some(vote_handler) = data.get_mut::<VoteHandler>() {
            vote_handler
                .vote_counts
                .insert(msg_id.get(), (msg.mentions[0].id.get(), 0));
        } else {
            println!("Error: VoteHandler not found in TypeMap.");
        }
    }
}

async fn votekick_vote(ctx: &Context, reaction: &Reaction) {
    if reaction.emoji == ReactionType::Unicode("✅".to_string()) {
        // get vote_handler from ctx.data
        // Begin accessing the data map
        let mut data = ctx.data.write().await;
        if let Some(vote_handler) = data.get_mut::<VoteHandler>() {
            let msg_id = reaction.message_id.get();
            // if message is not a vote -> return
            if !vote_handler.vote_counts.contains_key(&msg_id) {
                println!(
                    "Not a vote message. {:?} {}",
                    vote_handler.vote_counts, &msg_id
                );
                return;
            }
            let user_id = vote_handler.vote_counts.get(&msg_id).unwrap().0;
            let mut count = vote_handler.vote_counts.get_mut(&msg_id).unwrap().1;
            count += 1;

            if count >= 1 {
                vote_handler.vote_counts.insert(msg_id, (user_id, 0));

                // Get the guild ID and fetch the member
                if let Some(guild_id) = reaction.guild_id {
                    // Fetch the member object for the user
                    if let Ok(mut member) = guild_id.member(&ctx.http, user_id).await {
                        // Set the timeout duration (1 minute from now)
                        let timeout_duration = chrono::Utc::now() + chrono::Duration::minutes(1);

                        // Convert to serenity's Timestamp
                        let timeout_timestamp: serenity::model::Timestamp = timeout_duration.into();

                        // Disable communication for the specified duration
                        if let Err(why) = member
                            .disable_communication_until_datetime(&ctx.http, timeout_timestamp)
                            .await
                        {
                            println!("Error muting user: {:?}", why);
                        } else {
                            println!(
                                "{}: User <@{}> has been muted for 1 minute.",
                                date_helper::timestamp_string(),
                                user_id
                            );

                            // Notify the channel that the user has been muted
                            if let Err(why) = reaction
                                .channel_id
                                .say(
                                    &ctx.http,
                                    format!("<@{}> has been muted for 1 minute.", user_id),
                                )
                                .await
                            {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    } else {
                        println!("Could not find member in the guild.");
                    }
                } else {
                    println!("Reaction did not occur in a guild.");
                }
            } else {
                vote_handler.vote_counts.insert(msg_id, (user_id, count));
            }
        } else {
            println!("Error: VoteHandler not found in TypeMap.");
        }
    }
}

async fn init_vote_handler(data: &mut TypeMap) {
    data.insert::<VoteHandler>(VoteHandler {
        vote_counts: HashMap::new(),
    });
}

async fn init_type_map(data: &mut TypeMap) {
    init_vote_handler(data).await;
    // Add other handlers here
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
            votekick_init(&ctx, &msg).await;
        }
        // Other interactive commands here
        // else if (msg.content....) { }
    }

    // Bot (self) joins server event
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "{}: {} is connected!",
            date_helper::timestamp_string(),
            ready.user.name
        );

        // Initialize the VoteHandler in the TypeMap ("Bot chache")
        let mut data = ctx.data.write().await;
        init_type_map(&mut data).await;
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

        votekick_vote(&ctx, &reaction).await;
    }

    // ! if there are more uses for this event: extract user time tracking to a separate function
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
        | GatewayIntents::GUILD_MEMBERS
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
