use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

pub struct Database {
    client: redis::Client,
}

impl Database {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    pub async fn set(&mut self, key: String, value: String) -> redis::RedisResult<()> {
        if let Ok(mut connection) = self.client.get_connection() {
            connection.set(key, value)?;
        } else {
            panic!("Connection error.");
        }
        Ok(())
    }

    pub async fn get(&mut self, key: String) -> redis::RedisResult<String> {
        if let Ok(mut connection) = self.client.get_connection() {
            let value: String = connection.get(key).unwrap_or_default();
            Ok(value)
        } else {
            panic!("Connection error.");
        }
    }

    pub async fn exists(&mut self, key: String) -> bool {
        if let Ok(mut connection) = self.client.get_connection() {
            connection.exists(key).unwrap()
        } else {
            panic!("Connection error.");
        }
    }
}
