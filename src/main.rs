extern crate config;

use tbot::Bot;

mod actions;
mod handlers;
mod settings;
// mod database;

#[tokio::main]
async fn main() {
    let settings = settings::load();
    
    if settings.get_str("bot.token").is_err(){
        println!("Invalid configuration. please provide BOT token");
        return;
    }

    if settings.get_str("bot.master_id").is_err()  {
        println!("Invalid configuration. please provide Master ID");
        return;
    }

    let mut bot = Bot::new(settings.get_str("bot.token").unwrap()).stateful_event_loop(settings);

    bot.command("ban", actions::ban::ban);
    bot.command("mute", actions::mute::mute);
    bot.command("kick", actions::kick::kick);

    //TODO: Add help
    // bot.help(handler)

    bot.new_members(handlers::new_group::check_new_group);

    bot.polling().start().await.unwrap();
}
