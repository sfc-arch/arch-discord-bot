use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

pub struct RouletteClientData;

impl TypeMapKey for RouletteClientData {
    type Value = Arc<RouletteClient>;
}

pub struct RouletteClient {
    pub base_url: String,
}

impl RouletteClient {
    pub async fn create_roulette(&self, request: RouletteCreateRequest) -> RouletteCreateResponse {
        let client = reqwest::Client::new();

        match client
            .post(format!("{}/api/roulette", self.base_url))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
        {
            Ok(ok) => serde_json::from_str(&ok.text().await.unwrap()).unwrap(),
            Err(_) => {
                panic!("Error")
            }
        }
    }

    pub async fn start_roulette(&self, id: String, secret: String) {
        let client = reqwest::Client::new();

        let request = RouletteStartRequest { secret };

        match client
            .post(format!("{}/api/roulette/{}/start", self.base_url, id))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
        {
            Ok(_) => {}
            Err(_) => {
                panic!("Error")
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteItem {
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteCreateRequest {
    pub title: String,
    pub items: Vec<RouletteItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteCreateResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String, // UNIX Time
    pub secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteGetResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String,
    pub result: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteStartRequest {
    pub secret: String,
}
