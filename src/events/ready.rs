use serenity::{
    model::prelude::{
        command::{Command, CommandOptionType},
        Ready,
    },
    prelude::Context,
};

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let _ = Command::set_global_application_commands(&ctx.http, |commands| {
        commands.create_application_command(|command| {
            command
                .name("bento")
                .description("ARCH弁当")
                .create_option(|o| {
                    o.kind(CommandOptionType::SubCommandGroup)
                        .name("shop")
                        .description("店の管理")
                        .create_sub_option(|so| {
                            so.kind(CommandOptionType::SubCommand)
                                .name("add")
                                .description("店の追加")
                                .create_sub_option(|n| {
                                    n.kind(CommandOptionType::String)
                                        .name("name")
                                        .description("店名")
                                        .required(true)
                                })
                                .create_sub_option(|t| {
                                    t.kind(CommandOptionType::String)
                                        .name("url")
                                        .description("URL")
                                        .required(true)
                                })
                        })
                        .create_sub_option(|so| {
                            so.kind(CommandOptionType::SubCommand)
                                .name("remove")
                                .description("店の削除")
                                .create_sub_option(|n| {
                                    n.kind(CommandOptionType::String)
                                        .name("name")
                                        .description("店名")
                                        .required(true)
                                })
                        })
                        .create_sub_option(|so| {
                            so.kind(CommandOptionType::SubCommand)
                                .name("list")
                                .description("店一覧")
                        })
                })
        })
    })
    .await;
}
