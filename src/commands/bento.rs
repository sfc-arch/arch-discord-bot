use serenity::{
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::database::shop_database::{Shop, ShopDatabaseClientData};

pub async fn bento_command(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let storage_lock = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ShopDatabaseClientData>()
            .expect("Cannot get ShopDatabaseClientData")
            .clone()
    };

    let mut shop_db = storage_lock.lock().await;

    command
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|d| d.content("bento"))
        })
        .await?;
    Ok(())
}
