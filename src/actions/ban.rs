use std::sync::Arc;
use tbot::contexts;
use tbot::prelude::*;
use tbot::types::parameters::Text;
use tokio::sync::Mutex;

pub async fn ban(ctx: Arc<contexts::Command<contexts::Text>>, _state: Arc<Mutex<rusqlite::Connection>>) {
    if let Some(message) = ctx.reply_to.clone() {
        // Ban reason is important
        if ctx.text.value.is_empty() {
            ctx.send_message(Text::markdown_v2(
                "Please specify a reason to ban user `/ban {reason}`",
            ))
            .call()
            .await
            .unwrap();
            return;
        }
        let user_to_kick = message.from.as_ref().unwrap().id;
        let username = message.from.as_ref().unwrap().username.as_ref().unwrap();
        ctx.kick_chat_member(user_to_kick).call().await.unwrap();
        ctx.delete_this_message().call().await.unwrap();
        ctx.send_message(Text::markdown_v2(&format!(
            "Banned user {} for: \n{}",
            username, ctx.text.value
        )))
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
