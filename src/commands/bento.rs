use serenity::{
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn bento_command(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    command
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|d| d.content("bento test"))
        })
        .await?;
    Ok(())
}
