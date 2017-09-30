#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;
extern crate serde_json;

#[macro_use]
extern crate slog;
extern crate sloggers;

#[macro_use]
extern crate lazy_static;

extern crate serenity;

use std::fs::File;
use std::io::Read;
use std::env;

use serenity::prelude::*;
use serenity::framework::StandardFramework;
use serenity::framework::standard::help_commands;

mod logger;
#[macro_use]
mod utils;
mod music;
mod commands;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BotConfig {
    token: String,
}

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let config = read_config("discopotty.yaml");

    info!(logger::LOGGER, "Configuration:\n{:#?}", config);

    let mut client = Client::new(&config.token, Handler);

    client.with_framework(StandardFramework::new()
        .configure(|c| {
            c.prefix("!")
                .on_mention(true)
                .allow_dm(false)
        })
        .command("help", |c| {
            c.desc("Shows usage for all available commands\nCall 'help <command>' to get usage \
                       for specific command")
                .exec_help(help_commands::with_embeds)
        })
        .command("ping", |c| {
            c.desc("Responds with 'pong'")
                .exec(commands::utils::ping)
        })
        .command("play", |c| {
            c.known_as("pl")
                .desc("Pauses/Resumes playback if called without any arguments\nPlays specified \
                       link or query immediately if supplied")
                .exec(commands::music::playback::play)
        })
        .command("join", |c| {
            c.desc("Makes bot join the voice channel of caller or argument if provided")
                .exec(commands::voice::join)
        })
        .command("leave", |c| {
            c.desc("Makes bot leave the voice channel he's currently on")
                .exec(commands::voice::leave)
        }));

    info!(logger::LOGGER, "Starting bot...");
    if let Err(why) = client.start() {
        error!(logger::LOGGER, "{}", why);
    } else {
        info!(logger::LOGGER, "Running");
    }
}

fn read_config(path: &str) -> BotConfig {
    let mut config_file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            let mut file_path = env::current_dir().unwrap();
            file_path.push(path);
            error!(logger::LOGGER,
                   "Configuration file not found in '{}'",
                   file_path.display());
            std::process::exit(1)
        }
    };

    let mut config = String::new();
    config_file.read_to_string(&mut config)
        .expect("Couldn't read configuration file");

    serde_yaml::from_str(&config).expect("Invalid configuration file")
}
