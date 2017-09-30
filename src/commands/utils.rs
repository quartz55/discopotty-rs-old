use serenity::utils::MessageBuilder;
use serenity::model::Message;
use serenity::prelude::*;
use serenity::framework::standard::{CommandError, Args};
use std::result::Result;

use logger;

pub fn ping(ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
    let latency = ctx.shard.lock().latency().unwrap();
    let latency = latency.as_secs() as f64 * 1e3 + latency.subsec_nanos() as f64 * 1e-6;
    let pong = MessageBuilder::new()
        .push("Pong ")
        .mention(msg.author.id)
        .push("!")
        .push(format!(" ({:.0}ms)", latency))
        .build();
    send_msg!(msg.channel_id => &pong);
    Ok(())
}
