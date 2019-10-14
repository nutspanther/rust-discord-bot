use serenity::{client::Context};

use serenity::{
    framework::standard::{
        CommandResult,
        macros::{
            command
        }
    },
    model::{channel::Message},

};
use crate::{check_msg};

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx, "Pong!"));
    Ok(())
}