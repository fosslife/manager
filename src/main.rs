extern crate config;

use tbot::Bot;

mod actions;
mod handlers;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::load();
    // let settings = Arc::new(settings);

    let mut bot = Bot::new(settings.get_str("bot.token").unwrap()).stateful_event_loop(settings);

    bot.command("ban", actions::ban::ban);
    bot.command("mute", actions::mute::mute);
    bot.command("kick", actions::kick::kick);

    //TODO: Add help
    // bot.help(handler)

    bot.new_members(handlers::new_group::check_new_group);

    bot.polling().start().await.unwrap();
}
