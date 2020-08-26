use std::sync::Arc;
use tbot::contexts;
use tbot::prelude::*;
use tbot::types::chat::Permissions;

pub async fn mute(ctx: Arc<contexts::Command<contexts::Text>>, _state: Arc<config::Config>) {
    if let Some(message) = ctx.reply_to.clone() {
        let user_to_mute = message.from.as_ref().unwrap().id;
        let username = message.from.as_ref().unwrap().username.as_ref().unwrap();
        if let Err(mute_action) = ctx
            .restrict_chat_member(user_to_mute, Permissions::default())
            //TODO: remove hardcoded 40seconds
            .until_date(ctx.date + 40)
            .call()
            .await
        {
            ctx.send_message("could not mute user, check console for errors")
                .call()
                .await
                .unwrap();
            dbg!(mute_action);
        } else {
            ctx.send_message(&format!("Muted user user {}", username))
                .call()
                .await
                .unwrap();
        }
    } else {
        //TODO: implement /mute userid api
        ctx.send_message("Please reply to a message that you want to mute!")
            .call()
            .await
            .unwrap();
    }
}
