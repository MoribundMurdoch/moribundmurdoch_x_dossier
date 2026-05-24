use dioxus::prelude::*;

use crate::export::public_data::generate_public_posts_json;
use crate::ingest::x_api::XApiClient;

pub fn ImportPanel() -> Element {
    let mut status = use_signal(|| String::from("Maintainer tools ready."));
    let mut profile_preview = use_signal(|| String::new());
    let mut post_preview = use_signal(|| String::new());

    let test_x_api = move |_| {
        spawn(async move {
            status.set("Contacting the X API goblin hatch...".to_string());
            profile_preview.set(String::new());
            post_preview.set(String::new());

            let result = async {
                let username = std::env::var("X_USERNAME")
                    .unwrap_or_else(|_| "MoribundMurdoch".to_string());

                let client = XApiClient::from_env()?;
                let user = client.lookup_user_by_username(&username).await?;
                let posts = client.fetch_user_posts(&user.id, 10).await?;

                let metrics = user
                    .public_metrics
                    .as_ref()
                    .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
                    .unwrap_or_else(|| "No public metrics returned.".to_string());

                let profile = format!(
                    "Resolved @{username}\n\nid: {}\nname: {}\nusername: {}\nlocation: {}\ncreated_at: {}\nprotected: {:?}\nverified: {:?}\n\npublic_metrics:\n{}",
                    user.id,
                    user.name,
                    user.username,
                    user.location.unwrap_or_else(|| "None".to_string()),
                    user.created_at.unwrap_or_else(|| "Unknown".to_string()),
                    user.protected,
                    user.verified,
                    metrics,
                );

                let post_lines = posts
                    .iter()
                    .map(|post| {
                        let created = post
                            .created_at
                            .clone()
                            .unwrap_or_else(|| "unknown date".to_string());

                        let metrics = post
                            .public_metrics
                            .as_ref()
                            .map(|v| serde_json::to_string(v).unwrap_or_default())
                            .unwrap_or_default();

                        format!(
                            "- {} | {} | {}\n  {}",
                            post.id,
                            created,
                            metrics,
                            post.text.replace('\n', " ")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");

                anyhow::Ok((profile, post_lines))
            }
            .await;

            match result {
                Ok((profile, posts)) => {
                    status.set(
                        "Maintainer X API test succeeded. The public-data periscope has blinked."
                            .to_string(),
                    );
                    profile_preview.set(profile);
                    post_preview.set(posts);
                }
                Err(err) => {
                    status.set(format!("Maintainer X API test failed:\n{err:#}"));
                }
            }
        });
    };

    let generate_public_data = move |_: Event<MouseData>| {
        spawn(async move {
            status.set("Generating public_data/posts.json...".to_string());
            profile_preview.set(String::new());
            post_preview.set(String::new());

            match generate_public_posts_json().await {
                Ok(path) => {
                    status.set(format!("Generated {}", path.display()));
                }
                Err(err) => {
                    status.set(format!(
                        "Failed to generate public_data/posts.json:\n{err:#}"
                    ));
                }
            }
        });
    };

    rsx! {
        section {
            h2 { "Import / Sync" }
            p {
                "Maintainer-only tools for building the public MoribundMurdoch dataset. \
                 Public viewers should never need an API token."
            }

            div {
                style: "display: flex; gap: 10px; flex-wrap: wrap; margin-top: 16px;",

                button { "Import X Archive ZIP" }

                button {
                    onclick: test_x_api,
                    "Maintainer: Test X API"
                }

                button {
                    onclick: generate_public_data,
                    "Generate public_data/posts.json"
                }

                button { "Reconcile Sources" }
            }

            div {
                style: "
                    margin-top: 18px;
                    padding: 12px;
                    border: 1px solid #4a4058;
                    background: #24212c;
                    white-space: pre-wrap;
                ",
                "{status}"
            }

            if !profile_preview().is_empty() {
                div {
                    style: "
                        margin-top: 18px;
                        padding: 12px;
                        border: 1px solid #4a4058;
                        background: #15131a;
                        white-space: pre-wrap;
                    ",
                    h3 { "Resolved Profile" }
                    pre { "{profile_preview}" }
                }
            }

            if !post_preview().is_empty() {
                div {
                    style: "
                        margin-top: 18px;
                        padding: 12px;
                        border: 1px solid #4a4058;
                        background: #15131a;
                        white-space: pre-wrap;
                    ",
                    h3 { "Recent Posts" }
                    pre { "{post_preview}" }
                }
            }
        }
    }
}
