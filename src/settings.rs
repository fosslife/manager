use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct BotConfig {
    token: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ChatConfig {
    chats: Vec<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    bot: BotConfig,
    chat: ChatConfig,
}

pub fn load() -> Config {
    let default = Config::try_from(&AppConfig::default()).unwrap();
    let mut settings = Config::default();
    settings.merge(default).unwrap();
    settings.merge(File::with_name("config/default")).unwrap();
    settings.merge(File::with_name("config/local")).unwrap();
    settings
}
