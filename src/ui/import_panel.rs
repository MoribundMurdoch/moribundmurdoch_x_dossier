use dioxus::prelude::*;

use crate::ingest::x_api::XApiClient;

pub fn ImportPanel() -> Element {
    let mut status = use_signal(|| {
        String::from(
            "Maintainer tools ready.\n\nPublic viewers do not need an API token. \
             API and archive imports are only for generating the public dataset.",
        )
    });
    let mut profile_preview = use_signal(String::new);
    let mut post_preview = use_signal(String::new);

    let test_x_api = move |_| {
        spawn(async move {
            status.set("Contacting the X API goblin hatch...".to_string());
            profile_preview.set(String::new());
            post_preview.set(String::new());

            let result = async {
                let username =
                    std::env::var("X_USERNAME").unwrap_or_else(|_| "MoribundMurdoch".to_string());

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
                        "Maintainer X API test succeeded.\n\nThe public-data periscope has blinked."
                            .to_string(),
                    );
                    profile_preview.set(profile);
                    post_preview.set(posts);
                }
                Err(err) => {
                    status.set(format!(
                        "Maintainer X API test failed:\n{err:#}\n\nPublic viewers do not need this. \
                         This is only for the maintainer generating the dataset.\n\nAuth checklist:\n- Is .env in the project root?\n- Is X_BEARER_TOKEN real and not the placeholder?\n- Is the token copied without quotes or spaces?\n- Does your X developer app have access to this endpoint?"
                    ));
                }
            }
        });
    };

    let generate_public_data = move |_| {
        status.set(
            "Dataset generation placeholder.\n\nNext implementation step:\nX archive/API rows → normalized PublicPost records → public_data/posts.json\n\nNo public viewer API token should ever be required."
                .to_string(),
        );
        profile_preview.set(String::new());
        post_preview.set(String::new());
    };

    let import_archive = move |_| {
        status.set(
            "Archive ZIP importer placeholder.\n\nNext implementation step:\nOpen an X archive ZIP, scan its JSON files, normalize public posts/replies/quotes/reposts, then generate the public dataset."
                .to_string(),
        );
        profile_preview.set(String::new());
        post_preview.set(String::new());
    };

    let reconcile_sources = move |_| {
        status.set(
            "Reconciliation placeholder.\n\nFuture behavior:\nMerge archive ZIP history with API freshness, dedupe by post ID, preserve source provenance, then export public_data/*.json."
                .to_string(),
        );
        profile_preview.set(String::new());
        post_preview.set(String::new());
    };

    rsx! {
        section {
            style: "
                padding: 22px;
                border: 1px solid rgba(135, 178, 255, 0.22);
                border-radius: 24px;
                background: linear-gradient(180deg, rgba(20, 31, 58, 0.72), rgba(16, 25, 47, 0.62));
                box-shadow:
                    0 18px 40px rgba(0,0,0,0.24),
                    inset 0 1px 0 rgba(255,255,255,0.05);
            ",

            h2 {
                style: "margin: 0 0 8px; color: #82d9ff; text-shadow: 0 0 18px rgba(130,217,255,0.22);",
                "Maintainer Tools"
            }

            p {
                style: "margin: 0; color: #9bb0d3; line-height: 1.55;",
                "Private tools for building the public MoribundMurdoch dataset. \
                 Public viewers should never need an API token."
            }

            div {
                style: "
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
                    gap: 12px;
                    margin-top: 18px;
                ",

                MaintainerButton {
                    label: "Import X Archive ZIP",
                    detail: "historical dataset backfill",
                    onclick: import_archive
                }

                MaintainerButton {
                    label: "Maintainer: Test X API",
                    detail: "fresh public activity check",
                    onclick: test_x_api
                }

                MaintainerButton {
                    label: "Generate public_data/posts.json",
                    detail: "static viewer payload",
                    onclick: generate_public_data
                }

                MaintainerButton {
                    label: "Reconcile Sources",
                    detail: "archive + API merge",
                    onclick: reconcile_sources
                }
            }

            div {
                style: "
                    margin-top: 18px;
                    padding: 14px;
                    border: 1px solid rgba(135, 178, 255, 0.22);
                    border-radius: 20px;
                    background: rgba(10, 16, 31, 0.52);
                    color: #dce8ff;
                    white-space: pre-wrap;
                    line-height: 1.5;
                ",
                "{status}"
            }

            if !profile_preview().is_empty() {
                PreviewPanel {
                    title: "Resolved Profile",
                    body: profile_preview()
                }
            }

            if !post_preview().is_empty() {
                PreviewPanel {
                    title: "Recent Posts",
                    body: post_preview()
                }
            }
        }
    }
}

#[component]
fn MaintainerButton(
    label: &'static str,
    detail: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            onclick: move |event| onclick.call(event),
            style: "
                text-align: left;
                min-height: 86px;
                padding: 13px 14px;
                border: 1px solid rgba(135, 178, 255, 0.24);
                border-radius: 18px;
                background:
                    radial-gradient(circle at top right, rgba(130, 217, 255, 0.12), transparent 36%),
                    linear-gradient(180deg, rgba(40, 59, 106, 0.72), rgba(24, 38, 74, 0.72));
                color: #dce8ff;
                cursor: pointer;
                box-shadow:
                    0 10px 24px rgba(0,0,0,0.18),
                    inset 0 1px 0 rgba(255,255,255,0.05);
            ",

            span {
                style: "display: block; color: #f4f8ff; font-weight: 650;",
                "{label}"
            }

            span {
                style: "display: block; margin-top: 5px; color: #9bb0d3; font-size: 0.82rem;",
                "{detail}"
            }
        }
    }
}

#[component]
fn PreviewPanel(title: &'static str, body: String) -> Element {
    rsx! {
        div {
            style: "
                margin-top: 18px;
                padding: 14px;
                border: 1px solid rgba(135, 178, 255, 0.22);
                border-radius: 20px;
                background: rgba(8, 12, 24, 0.72);
                white-space: pre-wrap;
                box-shadow: inset 0 1px 0 rgba(255,255,255,0.04);
            ",

            h3 {
                style: "margin-top: 0; color: #c1a8ff;",
                "{title}"
            }

            pre {
                style: "
                    margin-bottom: 0;
                    color: #dce8ff;
                    white-space: pre-wrap;
                    word-break: break-word;
                    line-height: 1.45;
                ",
                "{body}"
            }
        }
    }
}
