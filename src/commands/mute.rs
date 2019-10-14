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
use crate::{check_msg, VoiceManager};

#[command]
fn mute(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match &ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx, "Groups and DMs not supported"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return Ok(());
        },
    };

    if handler.self_mute {
        check_msg(msg.channel_id.say(&ctx, "Already muted"));
    } else {
        handler.mute(true);

        check_msg(msg.channel_id.say(&ctx, "Now muted"));
    }
    Ok(())
}