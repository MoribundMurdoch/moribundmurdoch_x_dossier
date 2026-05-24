use dioxus::prelude::*;

use crate::public_data::sample::sample_posts;

pub fn TimelinePanel() -> Element {
    let posts = sample_posts();

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
                "Timeline"
            }

            p {
                style: "margin: 0; color: #9bb0d3; line-height: 1.55;",
                "A friendlier public archive of MoribundMurdoch's X/Twitter history. \
                Currently using sample data until the archive generator exists."
            }

            div {
                style: "
                    display: grid;
                    gap: 16px;
                    margin-top: 20px;
                ",

                for post in posts {
                    article {
                        key: "{post.id}",
                        style: "
                            position: relative;
                            overflow: hidden;
                            border: 1px solid rgba(135, 178, 255, 0.22);
                            border-radius: 22px;
                            background:
                                radial-gradient(circle at top right, rgba(130, 217, 255, 0.12), transparent 36%),
                                linear-gradient(180deg, rgba(31, 45, 80, 0.68), rgba(18, 28, 54, 0.62));
                            padding: 16px;
                            box-shadow:
                                0 14px 30px rgba(0,0,0,0.20),
                                inset 0 1px 0 rgba(255,255,255,0.05);
                        ",

                        div {
                            style: "
                                display: flex;
                                justify-content: space-between;
                                gap: 12px;
                                flex-wrap: wrap;
                                margin-bottom: 10px;
                            ",

                            div {
                                style: "color: #82d9ff; font-size: 0.88rem;",
                                "{post.created_at} · {post.kind:?}"
                            }

                            div {
                                style: "
                                    color: #9bb0d3;
                                    font-size: 0.78rem;
                                    text-transform: uppercase;
                                    letter-spacing: 0.12em;
                                ",
                                "public dataset row"
                            }
                        }

                        p {
                            style: "
                                margin: 0;
                                color: #f4f8ff;
                                line-height: 1.55;
                                font-size: 1rem;
                            ",
                            "{post.text}"
                        }

                        if !post.hashtags.is_empty() {
                            div {
                                style: "display: flex; gap: 8px; flex-wrap: wrap; margin-top: 12px;",

                                for hashtag in post.hashtags.clone() {
                                    span {
                                        key: "{hashtag}",
                                        style: "
                                            border: 1px solid rgba(130, 217, 255, 0.34);
                                            border-radius: 999px;
                                            color: #82d9ff;
                                            background: rgba(130, 217, 255, 0.08);
                                            padding: 4px 9px;
                                            font-size: 0.78rem;
                                        ",
                                        "#{hashtag}"
                                    }
                                }
                            }
                        }

                        if !post.broker_tags.is_empty() {
                            div {
                                style: "display: flex; gap: 8px; flex-wrap: wrap; margin-top: 10px;",

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

                        if let Some(warning) = post.llm_context_warning.clone() {
                            div {
                                style: "
                                    margin-top: 14px;
                                    border-left: 3px solid #8f96ff;
                                    border-radius: 0 12px 12px 0;
                                    padding: 10px 12px;
                                    color: #dce8ff;
                                    background: rgba(143, 150, 255, 0.09);
                                    font-size: 0.9rem;
                                    line-height: 1.45;
                                ",
                                strong {
                                    style: "color: #c1a8ff;",
                                    "LLM context warning: "
                                }
                                "{warning}"
                            }
                        }

                        if let Some(url) = post.url.clone() {
                            div {
                                style: "margin-top: 12px;",
                                a {
                                    href: "{url}",
                                    target: "_blank",
                                    style: "color: #82d9ff; text-decoration: none; font-size: 0.9rem;",
                                    "Open source post signal ↗"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
