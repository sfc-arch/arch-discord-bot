use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub prefix: String,
    pub token: String,
    pub redis_url: String,
    pub application_id: u64,
}
