use std::sync::Arc;
use tbot::contexts;
// use tbot::prelude::*;

pub async fn check_new_group(ctx: Arc<contexts::NewMembers>, state: Arc<config::Config>) {
    let chats = state
        .get::<Vec<tbot::types::chat::Id>>("chat.chat_list")
        .unwrap();
    let currnt_id = ctx.chat.id;
    match chats.clone().into_iter().find(|e| e == &currnt_id) {
        Some(chat) => println!("Valid addition {:?}", chat),
        None => {
            ctx.bot
                .send_message(ctx.chat.id, "This group is not part of network bye")
                .call()
                .await
                .unwrap();
            ctx.bot.leave_chat(ctx.chat.id).call().await.unwrap();
        }
    }
}
