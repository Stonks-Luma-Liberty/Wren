use dotenv::dotenv;
use futures::stream::TryStreamExt;
use log::info;
use reqwest::Client;
use serde::Deserialize;
use std::{collections::HashMap, error::Error};
use tokio::time::{sleep, Duration};
use twitter_v2::{authorization::BearerToken, Tweet, TwitterApi};

#[derive(Deserialize, Debug)]
struct Config {
    bearer_token: String,
    discord_webhook_url: String,
}

/// It takes a tweet, and posts it to a discord webhook.
///
/// Arguments:
///
/// * `client`: The Discord client
/// * `tweet`: The tweet that was just posted
/// * `webhook_url`: The webhook URL that you got from the Discord channel.
async fn post_tweet_to_discord(client: &Client, tweet: &Tweet, webhook_url: &String) {
    info!("{:?}", tweet);
    let mut map = HashMap::new();
    map.insert(
        "content",
        format!("https://twitter.com/web/status/{:?}", tweet.id.as_u64()),
    );
    client.post(webhook_url).json(&map).send().await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("Failed to read .env file");
    env_logger::init();
    info!("Starting wren bot");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    info!("Authenticating bot with Twitter API");
    let auth = BearerToken::new(&config.bearer_token);
    let api = TwitterApi::new(auth);

    let stream_rules: Vec<String> = api
        .get_tweets_search_stream_rules()
        .send()
        .await?
        .data()
        .unwrap()
        .into_iter()
        .map(|rule| rule.value.clone())
        .collect();
    info!("Stream rules {:?}", stream_rules);

    let client = &Client::new();
    let webhook_url = &config.discord_webhook_url.clone();

    let _stream = api
        .get_tweets_search_stream()
        .stream()
        .await?
        .try_for_each(|m| async move {
            if let Some(tweet) = m.data() {
                post_tweet_to_discord(client, tweet, webhook_url).await;
            }
            sleep(Duration::from_secs(3)).await;
            Ok(())
        })
        .await?;
    Ok(())
}
