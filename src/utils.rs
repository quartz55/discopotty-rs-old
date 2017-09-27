use serenity::model::{ChannelId, Guild};
use serenity::CACHE;
use std::sync::{Arc, RwLock};
use std::result::Result;

pub fn get_guild_cache<C: Into<ChannelId>>(chan: C) -> Result<Arc<RwLock<Guild>>, &'static str> {
    let cache = &*CACHE.read().unwrap();
    if let Some(channel) = cache.guild_channel(chan) {
        let guild_id = channel.read().unwrap().guild_id;
        if let Some(guild) = cache.guild(guild_id) {
            return Ok(guild);
        }
    }
    Err("what")
}

macro_rules! send_msg {
( $chan:expr => $msg:expr ) => ({
    send_msg!($chan => "{}", $msg)
});
( $chan:expr => $fmt:expr, $($arg:tt)+ ) => {{
    let msg = format!($fmt, $($arg)+);
    debug!(logger::LOGGER, "{}", msg);
    if let Err(why) = $chan.say(msg) {
        error!(logger::LOGGER, "Error sending message: {:?}", why);
    }
}}
}