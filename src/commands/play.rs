use serenity::{
    client::Context,
    voice
    };

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
use crate::dto::youtube::SearchResult;

#[command]
fn play(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut url = msg.content.clone();

    if !url.starts_with("http") {
       let search_string = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&key=AIzaSyCajcJYVzau-PNvg0WrFr4g6LbySgYGeVE", url);
       let response = reqwest::get(search_string.as_str())?.text()?;
        let r: SearchResult = serde_json::from_str(response.as_str())?;
        url = format!("https://www.youtube.com/watch?v={}", r.items[0].id.video_id);
    }

    let guild_id = match &ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx,"Error finding channel info"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ytdl(&url) {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx,"Error sourcing ffmpeg"));

                return Ok(());
            },
        };

        add_to_playlist(&url);
        check_msg(msg.channel_id.say(&ctx, format!("Adding {} to playlist", &url)));
        handler.play_only(source);

        check_msg(msg.channel_id.say(&ctx,"Playing song"));
    } else {
        check_msg(msg.channel_id.say(&ctx,"Not in a voice channel to play in"));
    }
    Ok(())
}

fn add_to_playlist(song: &String) {
    let mut playlist = Vec::new();    
    playlist.push(song)
}