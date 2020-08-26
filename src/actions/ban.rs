use std::sync::Arc;
use tbot::contexts;
use tbot::prelude::*;

pub async fn ban(ctx: Arc<contexts::Command<contexts::Text>>, _state: Arc<config::Config>) {
    if let Some(message) = ctx.reply_to.clone() {
        let user_to_kick = message.from.as_ref().unwrap().id;
        let username = message.from.as_ref().unwrap().username.as_ref().unwrap();
        ctx.kick_chat_member(user_to_kick).call().await.unwrap();
        ctx.send_message(&format!("Banned user {}", username))
            .call()
            .await
            .unwrap();
    } else {
        // TODO: add /ban userid API
        ctx.send_message("Please reply to a message that you want to Ban!")
            .call()
            .await
            .unwrap();
    }
}
