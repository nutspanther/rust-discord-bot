use serenity::{client::Context};

use serenity::{
    framework::standard::{
        CommandResult,
        macros::{
            command
        }
    },
    model::{channel::Message, misc::Mentionable},

};
use crate::{check_msg, VoiceManager};

#[command]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx) {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx, "Groups and DMs not supported"));

            return Ok(());
        }
    };

    let guild_id = guild.read().id;

    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx,"Not in a voice channel"));

            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect_to).is_some() {
        check_msg(msg.channel_id.say(&ctx,&format!("Joined {}", connect_to.mention())));
    } else {
        check_msg(msg.channel_id.say(&ctx,"Error joining the channel"));
    }
    Ok(())
}