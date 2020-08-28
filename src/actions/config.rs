use std::sync::Arc;
use tbot::contexts;
use tokio::sync::Mutex;
use std::env;
use tbot::prelude::*;

pub async fn config(ctx: Arc<contexts::Command<contexts::Text>>, _state: Arc<Mutex<rusqlite::Connection>>){
    println!("{} == {}", ctx.from.clone().unwrap().id.to_string(), env::var("GOVERNOR_ADMIN").unwrap());
    if ctx.from.clone().unwrap().id.to_string() != env::var("GOVERNOR_ADMIN").unwrap() {
        ctx.send_message("Only admins can issue this command").call().await.unwrap();
    };
}