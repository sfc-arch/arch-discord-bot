use crate::{
    bento::{BentoEvent, BentoInstanceData},
    commands::bento::bento_command,
    events,
};
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

        if let Interaction::MessageComponent(component_interaction) = interaction.clone() {
            let custom_id = component_interaction.data.custom_id.clone();

            if let Ok(event) = serde_json::from_str::<BentoEvent>(&custom_id) {
                let bento_instances = {
                    let data_read = ctx.data.read().await;
                    data_read
                        .get::<BentoInstanceData>()
                        .expect("Cannot get BentoInstanceData")
                        .clone()
                };
                let mut bento_instances = bento_instances.lock().await;

                if let Some(instance) = bento_instances.get_mut(&event.id) {
                    instance
                        .interaction(ctx, event, component_interaction)
                        .await;
                }
            }
        }
    }
}
