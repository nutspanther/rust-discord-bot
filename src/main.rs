//! Requires the "cache", "methods", and "voice" features be enabled in your
//! Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["cache", "framework", "standard_framework", "voice"]
//! ```
extern crate serenity;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod dto;
mod commands;

use std::{env, sync::Arc};

use serenity::client::bridge::voice::ClientVoiceManager;

use serenity::{client::Context, prelude::Mutex};

use serenity::{
    client::{Client, EventHandler},
    framework::standard::{
        StandardFramework,
        macros::{
            group
        }
    },
    model::{channel::Message, gateway::Ready},
    Result as SerenityResult,

};

use commands::{
    play::*,
    deafen::*,
    join::*,
    leave::*,
    mute::*,
    ping::*,
    undeafen::*,
    unmute::*,
};

use serenity::prelude::*;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

group!({
    name: "general",
    options: {},
    commands: [deafen, join, leave, mute, play, ping, undeafen, unmute]
});

fn main() {
    // Configure the client with your Discord bot token in the environment.
    env::set_var(
        "DISCORD_TOKEN",
        "NDM1NTAzMjU5NjA4ODA5NDgy.DbaApw.DyqYO6HL2s6N8niA332g7ji2sW8",
    );
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("/"))
            .group(&GENERAL_GROUP)
    );

    let _ = client
        .start()
        .map_err(|why| println!("Client ended: {:?}", why));
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
