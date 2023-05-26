use serenity::{
    model::prelude::{command::Command, Ready},
    prelude::Context,
};

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let _ = Command::set_global_application_commands(&ctx.http, |commands| {
        commands.create_application_command(|command| command.name("bento").description("ARCH弁当"))
    })
    .await;
}
