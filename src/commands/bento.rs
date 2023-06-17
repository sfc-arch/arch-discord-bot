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

    let sub_command = command.data.options[0].clone();

    match &*sub_command.name {
        "shop" => {
            let sub_command = sub_command.options[0].clone();

            match &*sub_command.name {
                "add" => {
                    let name = sub_command.options[0]
                        .clone()
                        .value
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    let url = sub_command.options[1]
                        .clone()
                        .value
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    let shop = Shop { name, url };

                    shop_db.add_shop(command.guild_id.unwrap().0, shop).await;

                    command
                        .create_interaction_response(&ctx.http, |f| {
                            f.interaction_response_data(|d| {
                                d.content("店を追加しました。").ephemeral(true)
                            })
                        })
                        .await?;
                }
                "remove" => {
                    let name = sub_command.options[0]
                        .clone()
                        .value
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    shop_db.remove_shop(command.guild_id.unwrap().0, name).await;

                    command
                        .create_interaction_response(&ctx.http, |f| {
                            f.interaction_response_data(|d| {
                                d.content("店を削除しました。").ephemeral(true)
                            })
                        })
                        .await?;
                }
                "list" => {
                    let shops = shop_db.get_shops(command.guild_id.unwrap().0).await;

                    let mut res = String::default();

                    for shop in shops {
                        res.push_str(&format!("- {}\n", shop.name));
                    }

                    command
                        .create_interaction_response(&ctx.http, |f| {
                            f.interaction_response_data(|d| d.content(res).ephemeral(true))
                        })
                        .await?;
                }
                _ => {}
            }
        }

        _ => {}
    }

    Ok(())
}
