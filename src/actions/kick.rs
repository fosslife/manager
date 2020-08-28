use std::sync::Arc;
use tbot::contexts;
use tbot::prelude::*;
use tokio::sync::Mutex;

pub async fn kick(ctx: Arc<contexts::Command<contexts::Text>>, _state: Arc<Mutex<rusqlite::Connection>>) {
    if let Some(message) = ctx.reply_to.clone() {
        let user_to_kick = message.from.as_ref().unwrap().id;
        let username = message.from.as_ref().unwrap().username.as_ref().unwrap();
        ctx.unban_chat_member(user_to_kick).call().await.unwrap();
        ctx.send_message(&format!("Kicked user {}", username))
            .call()
            .await
            .unwrap();
    } else {
        // TODO: add /kick userid API
        ctx.send_message("Please reply to a message that you want to kick!")
            .call()
            .await
            .unwrap();
    }
}
