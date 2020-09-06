use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub president: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatConfig {
    pub chats: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub bot: BotConfig,
    // pub chat: ChatConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let default_conf: AppConfig =
            toml::from_str(fs::read_to_string("config/default.toml").unwrap().as_str()).unwrap();
        AppConfig {
            // chat: ChatConfig {
            //     chats: default_conf.chat.chats,
            // },
            bot: BotConfig {
                token: default_conf.bot.token,
                president: default_conf.bot.president,
            },
        }
    }
}

pub fn load() -> AppConfig {
    let settings: AppConfig = toml::from_str(
        fs::read_to_string("config/local.toml")
            .unwrap_or_else(|x| {
                eprintln!("Error {}", x);
                std::process::exit(1);
            })
            .as_str(),
    )
    .expect("Incorrect local.toml file");
    info!("Settings loaded");
    settings
}
