use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BotConfig {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatConfig {
    chats: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    bot: BotConfig,
    chat: ChatConfig,
}

pub fn load() -> Config {
    let mut settings = Config::default();
    settings.merge(File::with_name("config/default")).unwrap();
    settings.merge(File::with_name("config/local")).unwrap();
    settings
}
