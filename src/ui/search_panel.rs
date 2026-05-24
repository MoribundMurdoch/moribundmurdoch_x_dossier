use dioxus::prelude::*;

use crate::public_data::sample::sample_posts;

pub fn SearchPanel() -> Element {
    let mut query = use_signal(String::new);

    let needle = query().to_lowercase();

    let results = sample_posts()
        .into_iter()
        .filter(|post| {
            needle.is_empty()
                || post.text.to_lowercase().contains(&needle)
                || post
                    .broker_tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&needle))
                || post
                    .hashtags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&needle))
                || post
                    .llm_context_warning
                    .as_ref()
                    .map(|warning| warning.to_lowercase().contains(&needle))
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>();

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
                "Search"
            }

            p {
                style: "margin: 0; color: #9bb0d3; line-height: 1.55;",
                "Search the public archive dataset. Currently using sample data."
            }

            div {
                style: "
                    margin-top: 16px;
                    padding: 14px;
                    border: 1px solid rgba(135, 178, 255, 0.22);
                    border-radius: 20px;
                    background: rgba(10, 16, 31, 0.42);
                ",

                label {
                    style: "display: block; margin-bottom: 8px; color: #c1a8ff; font-size: 0.86rem;",
                    "Archive Query"
                }

                input {
                    value: "{query}",
                    oninput: move |event| query.set(event.value()),
                    placeholder: "Search posts, tags, topics, LLM warnings...",
                    style: "
                        width: 100%;
                        border: 1px solid rgba(135, 178, 255, 0.26);
                        border-radius: 16px;
                        background: rgba(8, 12, 24, 0.78);
                        color: #dce8ff;
                        padding: 12px 14px;
                        outline: none;
                        box-shadow: inset 0 1px 0 rgba(255,255,255,0.04);
                    "
                }

                div {
                    style: "margin-top: 10px; color: #9bb0d3; font-size: 0.84rem;",
                    "{results.len()} matching signal row(s)"
                }
            }

            div {
                style: "display: grid; gap: 14px; margin-top: 18px;",

                for post in results {
                    article {
                        key: "{post.id}",
                        style: "
                            border: 1px solid rgba(135, 178, 255, 0.22);
                            border-radius: 20px;
                            background:
                                radial-gradient(circle at top right, rgba(193, 168, 255, 0.11), transparent 38%),
                                linear-gradient(180deg, rgba(31, 45, 80, 0.62), rgba(18, 28, 54, 0.58));
                            padding: 14px;
                            box-shadow:
                                0 12px 26px rgba(0,0,0,0.18),
                                inset 0 1px 0 rgba(255,255,255,0.05);
                        ",

                        div {
                            style: "display: flex; justify-content: space-between; gap: 12px; flex-wrap: wrap;",
                            strong {
                                style: "color: #82d9ff;",
                                "{post.created_at}"
                            }

                            span {
                                style: "color: #9bb0d3; font-size: 0.84rem;",
                                "{post.kind:?}"
                            }
                        }

                        p {
                            style: "margin: 10px 0 0; color: #f4f8ff; line-height: 1.5;",
                            "{post.text}"
                        }

                        if !post.broker_tags.is_empty() {
                            div {
                                style: "display: flex; gap: 8px; flex-wrap: wrap; margin-top: 12px;",

                                for tag in post.broker_tags.clone() {
                                    span {
                                        key: "{tag}",
                                        style: "
                                            border: 1px solid rgba(193, 168, 255, 0.36);
                                            border-radius: 999px;
                                            color: #c1a8ff;
                                            background: rgba(193, 168, 255, 0.08);
                                            padding: 4px 9px;
                                            font-size: 0.78rem;
                                        ",
                                        "{tag}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
