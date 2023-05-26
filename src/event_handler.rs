use crate::{commands::bento::bento_command, events};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, prelude::interaction::Interaction},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        events::ready::ready(ctx, ready).await
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            let name = &*command.data.name;
            match name {
                "bento" => bento_command(&ctx, &command).await.unwrap(),
                _ => {}
            }
        }
    }
}
