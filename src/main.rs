use std::env;
use tbot::Bot;
use tokio::sync::Mutex;

mod actions;
// mod handlers;
mod database;

#[tokio::main]
async fn main() {
    let db = database::load().await.unwrap();
    let admin = env::var("GOVERNOR_ADMIN");
    let token = env::var("GOVERNOR_TOKEN");

    if token.is_err() {
        println!("Invalid configuration. please provide BOT token");
        return;
    }

    if admin.is_err() {
        println!("Invalid configuration. please provide at least one Master ID");
        return;
    }

    let mut bot = Bot::new(token.unwrap()).stateful_event_loop(Mutex::new(db));

    // bot.new_members(handlers::new_group::check_new_group);
    bot.command("ban", actions::ban::ban);
    bot.command("mute", actions::mute::mute);
    bot.command("kick", actions::kick::kick);
    bot.command("config", actions::config::config);

    //TODO: Add help
    // bot.help(handler)

    bot.polling().start().await.unwrap();
}
