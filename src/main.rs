mod commands;
mod config;
mod database;
mod event_handler;
mod events;
mod implement;

use std::{env, sync::Arc};

use config::Config;
use database::shop_database::{ShopDatabase, ShopDatabaseClientData};
use event_handler::Handler;
use serenity::{
    client::Client, framework::StandardFramework, futures::lock::Mutex, prelude::GatewayIntents,
};

use songbird::SerenityInit;

/// Create discord client
///
/// Example:
/// ```rust
/// let client = create_client("!", "BOT_TOKEN", 123456789123456789).await;
///
/// client.start().await;
/// ```
async fn create_client(prefix: &str, token: &str, id: u64) -> Result<Client, serenity::Error> {
    let framework = StandardFramework::new().configure(|c| c.with_whitespace(true).prefix(prefix));

    Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .application_id(id)
        .framework(framework)
        .register_songbird()
        .await
}

#[tokio::main]
async fn main() {
    // Load config
    let config = {
        let config = std::fs::read_to_string("./config.toml");
        if let Ok(config) = config {
            toml::from_str::<Config>(&config).expect("Cannot load config file.")
        } else {
            let token = env::var("ARCH_TOKEN").unwrap();
            let application_id = env::var("ARCH_APP_ID").unwrap();
            let prefix = env::var("ARCH_PREFIX").unwrap();
            let redis_url = env::var("ARCH_REDIS_URL").unwrap();

            Config {
                token,
                application_id: u64::from_str_radix(&application_id, 10).unwrap(),
                redis_url,
                prefix,
            }
        }
    };

    // Create discord client
    let mut client = create_client(&config.prefix, &config.token, config.application_id)
        .await
        .expect("Err creating client");

    // Create database client
    let shop_database_client = {
        let redis_client = redis::Client::open(config.redis_url).unwrap();
        ShopDatabase::new(redis_client)
    };

    {
        let mut data = client.data.write().await;
        data.insert::<ShopDatabaseClientData>(Arc::new(Mutex::new(shop_database_client)));
    }

    // Run client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
