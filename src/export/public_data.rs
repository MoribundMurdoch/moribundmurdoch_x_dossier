use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use serde::Serialize;

use crate::ingest::x_api::{XApiClient, XTweet};

const PUBLIC_DATA_DIR: &str = "public_data";
const PUBLIC_POSTS_FILE: &str = "posts.json";

#[derive(Debug, Clone, Serialize)]
pub struct PublicPostsDocument {
    pub generated_at: String,
    pub source: PublicPostsSource,
    pub posts: Vec<PublicPost>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicPostsSource {
    pub platform: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicPost {
    pub id: String,
    pub url: String,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<serde_json::Value>,
}

pub async fn generate_public_posts_json() -> Result<PathBuf> {
    dotenvy::dotenv().ok();

    let username = std::env::var("X_USERNAME").unwrap_or_else(|_| "MoribundMurdoch".to_string());

    let client = XApiClient::from_env()?;
    let user = client.lookup_user_by_username(&username).await?;
    let posts = client.fetch_user_posts(&user.id, 100).await?;

    write_public_posts_json(&username, &posts)
}

pub fn write_public_posts_json(username: &str, posts: &[XTweet]) -> Result<PathBuf> {
    let output_dir = Path::new(PUBLIC_DATA_DIR);
    fs::create_dir_all(output_dir).context("Failed to create public_data directory")?;

    let output_path = output_dir.join(PUBLIC_POSTS_FILE);

    let document = PublicPostsDocument {
        generated_at: Utc::now().to_rfc3339(),
        source: PublicPostsSource {
            platform: "X".to_string(),
            username: username.to_string(),
        },
        posts: posts
            .iter()
            .map(|post| PublicPost {
                id: post.id.clone(),
                url: format!("https://x.com/{username}/status/{}", post.id),
                text: post.text.clone(),
                created_at: post.created_at.clone(),
                lang: post.lang.clone(),
                public_metrics: post.public_metrics.clone(),
            })
            .collect(),
    };

    let json =
        serde_json::to_string_pretty(&document).context("Failed to serialize public posts JSON")?;

    fs::write(&output_path, json).with_context(|| {
        format!(
            "Failed to write public posts JSON to {}",
            output_path.display()
        )
    })?;

    Ok(output_path)
}
