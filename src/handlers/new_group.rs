use std::sync::Arc;
use tbot::contexts;
// use tbot::prelude::*;

pub async fn check_new_group(_ctx: Arc<contexts::NewMembers>, _db: Arc<rusqlite::Connection>) {
    // let chats = db
    //     .get("chat")
    //     .unwrap();
    // let currnt_id = ctx.chat.id;
    // match chats.clone().into_iter().find(|e| e == &currnt_id) {
    //     Some(chat) => println!("Valid addition {:?}", chat),
    //     None => {
    //         ctx.bot
    //             .send_message(ctx.chat.id, "This group is not part of network bye")
    //             .call()
    //             .await
    //             .unwrap();
    //         ctx.bot.leave_chat(ctx.chat.id).call().await.unwrap();
    //     }
    // }
}
