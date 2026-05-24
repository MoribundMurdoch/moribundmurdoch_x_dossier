use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use serde::Deserialize;

const X_API_BASE: &str = "https://api.x.com/2";

#[derive(Debug, Clone)]
pub struct XApiClient {
    http: Client,
    bearer_token: String,
}

impl XApiClient {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let bearer_token = std::env::var("X_BEARER_TOKEN")
            .context("Missing X_BEARER_TOKEN in environment or .env file")?;

        Ok(Self {
            http: Client::new(),
            bearer_token,
        })
    }

    pub async fn lookup_user_by_username(&self, username: &str) -> Result<XUser> {
        let url = format!("{X_API_BASE}/users/by/username/{username}");

        let response = self
            .http
            .get(url)
            .bearer_auth(&self.bearer_token)
            .query(&[
                (
                    "user.fields",
                    "created_at,description,entities,id,location,name,profile_banner_url,profile_image_url,protected,public_metrics,url,username,verified",
                ),
            ])
            .send()
            .await
            .context("Failed to send X user lookup request")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Failed to read X user lookup response body")?;

        if !status.is_success() {
            return Err(anyhow!("X user lookup failed with {status}: {body}"));
        }

        let parsed: XUserLookupResponse =
            serde_json::from_str(&body).context("Failed to parse X user lookup response JSON")?;

        Ok(parsed.data)
    }

    pub async fn fetch_user_posts(&self, user_id: &str, max_results: usize) -> Result<Vec<XTweet>> {
        let url = format!("{X_API_BASE}/users/{user_id}/tweets");

        let max_results = max_results.clamp(5, 100).to_string();

        let response = self
            .http
            .get(url)
            .bearer_auth(&self.bearer_token)
            .query(&[
                ("max_results", max_results.as_str()),
                (
                    "tweet.fields",
                    "attachments,author_id,context_annotations,conversation_id,created_at,edit_history_tweet_ids,entities,id,in_reply_to_user_id,lang,public_metrics,referenced_tweets,source,text",
                ),
                (
                    "expansions",
                    "attachments.media_keys,author_id,referenced_tweets.id,referenced_tweets.id.author_id",
                ),
                (
                    "media.fields",
                    "duration_ms,height,media_key,preview_image_url,type,url,width,public_metrics,alt_text",
                ),
                (
                    "user.fields",
                    "created_at,description,id,location,name,profile_image_url,protected,public_metrics,url,username,verified",
                ),
            ])
            .send()
            .await
            .context("Failed to send X user posts request")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Failed to read X user posts response body")?;

        if !status.is_success() {
            return Err(anyhow!("X user posts request failed with {status}: {body}"));
        }

        let parsed: XTweetListResponse =
            serde_json::from_str(&body).context("Failed to parse X user posts response JSON")?;

        Ok(parsed.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct XUserLookupResponse {
    pub data: XUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XTweetListResponse {
    pub data: Option<Vec<XTweet>>,
    pub meta: Option<serde_json::Value>,
    pub includes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XUser {
    pub id: String,
    pub name: String,
    pub username: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub location: Option<String>,

    #[serde(default)]
    pub created_at: Option<String>,

    #[serde(default)]
    pub profile_image_url: Option<String>,

    #[serde(default)]
    pub profile_banner_url: Option<String>,

    #[serde(default)]
    pub public_metrics: Option<serde_json::Value>,

    #[serde(default)]
    pub verified: Option<bool>,

    #[serde(default)]
    pub protected: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XTweet {
    pub id: String,
    pub text: String,

    #[serde(default)]
    pub author_id: Option<String>,

    #[serde(default)]
    pub created_at: Option<String>,

    #[serde(default)]
    pub conversation_id: Option<String>,

    #[serde(default)]
    pub in_reply_to_user_id: Option<String>,

    #[serde(default)]
    pub lang: Option<String>,

    #[serde(default)]
    pub public_metrics: Option<serde_json::Value>,

    #[serde(default)]
    pub entities: Option<serde_json::Value>,

    #[serde(default)]
    pub referenced_tweets: Option<Vec<serde_json::Value>>,

    #[serde(default)]
    pub context_annotations: Option<Vec<serde_json::Value>>,
}
