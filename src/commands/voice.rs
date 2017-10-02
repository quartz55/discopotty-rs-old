use serenity::utils::MessageBuilder;
use serenity::model::{Message, Guild};
use serenity::prelude::*;
use serenity::framework::standard::{CommandError, Args};
use std::sync::Arc;

use logger;
use utils;

pub fn join(ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
    if let Ok(guild) = utils::get_guild_cache(msg.channel_id) {
        let guild = guild.read().unwrap();
        match args.len() {
            0 => {
                if let Some(voice) = guild.voice_states.get(&msg.author.id) {
                    ctx.shard
                        .lock()
                        .manager
                        .join(guild.id, voice.channel_id.unwrap());
                } else {
                    let reply = MessageBuilder::new()
                        .push_bold("ERROR: ")
                        .push("You have to join a voice channel first ")
                        .mention(msg.author.id)
                        .push("!")
                        .build();
                    send_msg!(msg.channel_id => &reply);
                }
            }
            1 => {
                let channel_name: String = args.single_n().unwrap();
                send_msg!(msg.channel_id => &channel_name);
            }
            _ => panic!("woops"),
        }
    }
    Ok(())
}

pub fn leave(ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
    let guild = Arc::clone(&msg.guild().unwrap());
    let guild: &Guild = &(*guild.read().unwrap());
    ctx.shard
        .lock()
        .manager
        .leave(guild.id);
    Ok(())
}
