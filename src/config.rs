use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bearer_token: String,
    pub discord_webhook_url: String,
}
