#![allow(dead_code)]
use rusqlite::Connection;
use settings::AppConfig;
use tbot::Bot;
use tokio::sync::Mutex;

mod actions;
mod database;
mod handlers;
mod settings;

pub struct Storages {
    db: Connection,
    config: AppConfig,
}

#[tokio::main]
async fn main() {
    let config = settings::load();
    // println!("Settings {:?}", config.get_str("bot.token"));
    let db = database::load().await.unwrap();
    let admin = config.bot.president.clone(); //config.get_str("bot.president");
    let token = config.bot.token.clone(); //config.get_str("bot.token");

    let storage = Storages { db, config };

    if token.is_empty() {
        println!("Invalid configuration. please provide BOT token");
        return;
    }

    if admin.is_empty() {
        println!("Invalid configuration. please provide at least one Master ID");
        return;
    }

    let mut bot = Bot::new(token).stateful_event_loop(Mutex::new(storage));

    bot.new_members(handlers::new_group::new_memeber);
    // bot.command("ban", actions::ban::ban);
    // bot.command("mute", actions::mute::mute);
    // bot.command("kick", actions::kick::kick);
    bot.command("promote", actions::promote::promote);
    bot.command("config", actions::config::config);

    //TODO: Add help
    // bot.help(handler)

    bot.polling().start().await.unwrap();
}
