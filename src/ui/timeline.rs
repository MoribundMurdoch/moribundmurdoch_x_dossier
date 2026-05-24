use dioxus::prelude::*;

use crate::public_data::sample::sample_posts;

pub fn TimelinePanel() -> Element {
    let posts = sample_posts();

    rsx! {
        section {
            h2 { "Timeline" }
            p {
                "A friendlier public archive of MoribundMurdoch's X/Twitter history. \
                Currently using sample data until the archive generator exists."
            }

            div {
                style: "display: grid; gap: 14px; margin-top: 18px;",

                for post in posts {
                    article {
                        key: "{post.id}",
                        style: "
                            border: 1px solid #4a4058;
                            background: #24212c;
                            padding: 14px;
                        ",

                        div {
                            style: "color: #bc8d6b; font-size: 0.9rem; margin-bottom: 8px;",
                            "{post.created_at} · {post.kind:?}"
                        }

                        p {
                            style: "line-height: 1.45;",
                            "{post.text}"
                        }

                        if !post.broker_tags.is_empty() {
                            div {
                                style: "display: flex; gap: 8px; flex-wrap: wrap; margin-top: 10px;",

                                for tag in post.broker_tags {
                                    span {
                                        key: "{tag}",
                                        style: "
                                            border: 1px solid #bc8d6b;
                                            color: #bc8d6b;
                                            padding: 3px 8px;
                                            font-size: 0.8rem;
                                        ",
                                        "{tag}"
                                    }
                                }
                            }
                        }

                        if let Some(warning) = post.llm_context_warning {
                            div {
                                style: "
                                    margin-top: 12px;
                                    border-left: 3px solid #a9aae2;
                                    padding-left: 10px;
                                    color: #b9aa91;
                                    font-size: 0.9rem;
                                ",
                                "LLM context warning: {warning}"
                            }
                        }
                    }
                }
            }
        }
    }
}
