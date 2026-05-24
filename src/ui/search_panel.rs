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
        })
        .collect::<Vec<_>>();

    rsx! {
        section {
            h2 { "Search" }
            p { "Search the public archive dataset. Currently using sample data." }

            input {
                value: "{query}",
                oninput: move |event| query.set(event.value()),
                placeholder: "Search posts, tags, topics, LLM warnings...",
                style: "
                    width: 100%;
                    max-width: 720px;
                    border: 1px solid #4a4058;
                    background: #15131a;
                    color: #eee5d2;
                    padding: 10px;
                    margin: 12px 0 18px;
                "
            }

            div {
                style: "display: grid; gap: 12px;",

                for post in results {
                    article {
                        key: "{post.id}",
                        style: "
                            border: 1px solid #4a4058;
                            background: #24212c;
                            padding: 12px;
                        ",

                        strong { "{post.created_at}" }
                        p { "{post.text}" }
                    }
                }
            }
        }
    }
}
