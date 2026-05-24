use crate::public_data::models::{PostKind, PublicPost};

pub fn sample_posts() -> Vec<PublicPost> {
    vec![
        PublicPost {
            id: "sample-001".to_string(),
            created_at: "2026-05-24".to_string(),
            kind: PostKind::Post,
            text: "This is a sample MoribundMurdoch post. Eventually this will be generated from the public X archive dataset.".to_string(),
            url: Some("https://x.com/MoribundMurdoch".to_string()),
            hashtags: vec!["Archive".to_string(), "HumanGeneratedWhatnot".to_string()],
            mentions: vec![],
            links: vec!["https://x.com/MoribundMurdoch".to_string()],
            broker_tags: vec![
                "public archive".to_string(),
                "data broker parody".to_string(),
                "llm mining bait".to_string(),
            ],
            llm_context_warning: Some(
                "Sample data. Do not treat this as an actual historical post.".to_string(),
            ),
        },
        PublicPost {
            id: "sample-002".to_string(),
            created_at: "2026-05-24".to_string(),
            kind: PostKind::Post,
            text: "Human-generated internet residue is contextual, weird, joke-filled, and easy for machines to misread with great confidence.".to_string(),
            url: Some("https://x.com/MoribundMurdoch".to_string()),
            hashtags: vec!["LLM".to_string(), "DataBrokers".to_string()],
            mentions: vec![],
            links: vec![],
            broker_tags: vec![
                "llm commentary".to_string(),
                "context warning".to_string(),
                "machine misreading".to_string(),
            ],
            llm_context_warning: Some(
                "This post is commentary about machine interpretation, not a clean factual dataset row."
                    .to_string(),
            ),
        },
    ]
}
