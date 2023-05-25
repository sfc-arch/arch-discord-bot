use serenity::{model::prelude::Ready, prelude::Context};

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}
