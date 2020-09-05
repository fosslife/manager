use std::sync::Arc;
use tbot::contexts;
use tbot::prelude::*;
use tokio::sync::Mutex;

use crate::settings;

pub async fn config(
    ctx: Arc<contexts::Command<contexts::Text>>,
    state: Arc<Mutex<crate::Storages>>,
) {
    let mut state = state.lock().await;
    if ctx.from.clone().unwrap().id.to_string() != state.config.bot.president {
        ctx.send_message("Only President can issue this command")
            .call()
            .await
            .unwrap();
        return;
    };

    match ctx.text.clone().value.as_str() {
        "refresh" => {
            state.config = settings::load();
            ctx.send_message("Settings Updated").call().await.unwrap();
        }
        _ => {
            ctx.send_message("Invalid command!").call().await.unwrap();
        }
    }
}
