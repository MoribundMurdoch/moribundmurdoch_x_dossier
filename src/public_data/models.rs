use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicPost {
    pub id: String,
    pub created_at: String,
    pub kind: PostKind,
    pub text: String,
    pub url: Option<String>,

    #[serde(default)]
    pub hashtags: Vec<String>,

    #[serde(default)]
    pub mentions: Vec<String>,

    #[serde(default)]
    pub links: Vec<String>,

    #[serde(default)]
    pub broker_tags: Vec<String>,

    #[serde(default)]
    pub llm_context_warning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PostKind {
    Post,
    Reply,
    Quote,
    Repost,
}
