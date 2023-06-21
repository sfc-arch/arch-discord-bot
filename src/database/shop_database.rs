use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::{futures::lock::Mutex, prelude::TypeMapKey};

use super::database::Database;

pub struct ShopDatabaseClientData;

impl TypeMapKey for ShopDatabaseClientData {
    type Value = Arc<Mutex<ShopDatabase>>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Shop {
    pub name: String,
    pub url: String,
}

pub struct ShopDatabase {
    database: Database,
}

impl ShopDatabase {
    pub fn new(client: redis::Client) -> Self {
        Self {
            database: Database::new(client),
        }
    }

    pub async fn add_shop(&mut self, server_id: u64, shop: Shop) {
        let mut shops = self.get_shops(server_id).await;
        if !shops.contains(&shop) {
            shops.push(shop);
        }
        self.set_shops(server_id, shops).await;
    }

    pub async fn remove_shop(&mut self, server_id: u64, name: String) {
        let mut shops = self.get_shops(server_id).await;
        let mut i = None;

        for (index, f_shop) in shops.iter().enumerate() {
            if name == *f_shop.name {
                i = Some(index);
                break;
            }
        }

        if let Some(index) = i {
            shops.remove(index);
        }

        self.set_shops(server_id, shops).await;
    }

    pub async fn get_shops(&mut self, server_id: u64) -> Vec<Shop> {
        serde_json::from_str(
            &self
                .database
                .get(server_id.to_string())
                .await
                .unwrap_or("{}".to_string()),
        )
        .unwrap_or(vec![])
    }

    pub async fn set_shops(&mut self, server_id: u64, shops: Vec<Shop>) {
        self.database
            .set(
                server_id.to_string(),
                serde_json::to_string(&shops).unwrap(),
            )
            .await
            .unwrap();
    }
}
