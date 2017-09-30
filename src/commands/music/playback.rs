// use serenity::utils::{MessageBuilder};
// use serenity::prelude::*;

use logger;
use utils;
use music::ytdl::{YtdlResult, parse_query_or_link};
use serenity::model::Message;
use serenity::prelude::*;
use serenity::framework::standard::{CommandError, Args};
use serenity::voice::ffmpeg;

pub fn play(ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
    let guild = utils::get_guild_cache(msg.channel_id).unwrap();
    let guild = guild.read().unwrap();
    if let Some(handler) = ctx.shard.lock().manager.get(guild.id) {
        if args.is_empty() {
            send_msg!(msg.channel_id => "Pause/Resume");
            handler.stop();
        } else {
            let query_or_link = args.full();
            let info = parse_query_or_link(&query_or_link).unwrap();
            let song = info.first().unwrap();
            if let &YtdlResult::Track(ref track) = song {
                println!("{:#?}", track);
                let source = ffmpeg(&track.url).unwrap();
                handler.play(source);
                send_msg!(msg.channel_id => "Playing: {}", &track.title);
            };
        }
    }
    Ok(())
}
