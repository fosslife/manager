extern crate config;

use config::{Config, File};
use serde::Deserialize;
// use std::collections::HashMap;
use tbot::Bot;

mod actions;
#[derive(Debug, Deserialize)]
struct BotConfig {
    token: String,
}
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    bot: BotConfig,
}

#[tokio::main]
async fn main() {
    let mut settings = Config::default();
    settings.merge(File::with_name("config/default")).unwrap();
    settings.merge(File::with_name("config/local")).unwrap();

    let appconfig = settings.try_into::<AppConfig>().unwrap();

    let mut bot = Bot::new(appconfig.bot.token).event_loop();

    bot.command("ban", actions::ban::ban);

    bot.command("mute", actions::mute::mute);

    bot.polling().start().await.unwrap();
}
