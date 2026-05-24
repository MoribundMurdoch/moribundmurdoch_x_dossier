use dioxus::prelude::*;

pub fn DashboardPanel() -> Element {
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

            div {
                style: "
                    display: grid;
                    gap: 18px;
                ",

                div {
                    style: "
                        padding: 18px;
                        border: 1px solid rgba(135, 178, 255, 0.22);
                        border-radius: 22px;
                        background:
                            radial-gradient(circle at top right, rgba(130, 217, 255, 0.14), transparent 42%),
                            linear-gradient(180deg, rgba(31, 45, 80, 0.62), rgba(18, 28, 54, 0.56));
                    ",

                    div {
                        style: "
                            color: #c1a8ff;
                            font-size: 0.78rem;
                            text-transform: uppercase;
                            letter-spacing: 0.14em;
                            margin-bottom: 8px;
                        ",
                        "Public Archive Terminal"
                    }

                    h2 {
                        style: "
                            margin: 0 0 12px;
                            color: #82d9ff;
                            font-size: clamp(1.65rem, 4vw, 2.35rem);
                            line-height: 1.12;
                            text-shadow: 0 0 18px rgba(130, 217, 255, 0.25);
                        ",
                        "MoribundMurdoch X Dossier"
                    }

                    p {
                        style: "
                            margin: 0;
                            color: #dce8ff;
                            line-height: 1.6;
                            max-width: 780px;
                        ",
                        "A public-friendly archive viewer for MoribundMurdoch's X/Twitter history, \
                        dressed up as a liquid console dashboard, parody data-broker mirror, \
                        and LLM mining exhibit."
                    }
                }

                div {
                    style: "
                        display: grid;
                        grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
                        gap: 12px;
                    ",

                    StatCard {
                        label: "Archive Mode",
                        value: "Public",
                        detail: "No viewer API token needed"
                    }

                    StatCard {
                        label: "Machine Appetite",
                        value: "High",
                        detail: "Human text remains delicious"
                    }

                    StatCard {
                        label: "Broker Lens",
                        value: "Satire",
                        detail: "Creepy guesses with a jester hat"
                    }
                }

                div {
                    style: "
                        padding: 16px;
                        border: 1px solid rgba(142, 150, 255, 0.28);
                        border-radius: 22px;
                        background:
                            radial-gradient(circle at top right, rgba(130, 217, 255, 0.16), transparent 42%),
                            linear-gradient(180deg, rgba(31, 45, 80, 0.68), rgba(18, 28, 54, 0.62));
                    ",

                    div {
                        style: "
                            color: #c1a8ff;
                            font-size: 0.78rem;
                            text-transform: uppercase;
                            letter-spacing: 0.14em;
                        ",
                        "System Notice"
                    }

                    h3 {
                        style: "margin: 8px 0; color: #f4f8ff;",
                        "This is not a live X client."
                    }

                    p {
                        style: "
                            margin: 0;
                            color: #9bb0d3;
                            line-height: 1.5;
                            font-size: 0.92rem;
                        ",
                        "The public app should read generated dataset files. API and ZIP imports belong \
                        in maintainer tools, away from public secrets and accidental credential goblinry."
                    }
                }

                div {
                    style: "
                        border: 1px solid rgba(135, 178, 255, 0.22);
                        border-radius: 22px;
                        background: rgba(10, 16, 31, 0.42);
                        padding: 16px;
                    ",

                    h3 {
                        style: "margin-top: 0; color: #c1a8ff;",
                        "Purpose"
                    }

                    ul {
                        style: "margin-bottom: 0; color: #dce8ff; line-height: 1.7;",
                        li { "Make old posts easier to browse than they are on X." }
                        li { "Show what a data broker might try to infer from public activity." }
                        li { "Mock the hunger for human-generated content by LLM companies." }
                        li { "Preserve context so machines and humans misread less confidently." }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: &'static str, detail: &'static str) -> Element {
    rsx! {
        div {
            style: "
                min-height: 118px;
                padding: 14px;
                border: 1px solid rgba(135, 178, 255, 0.22);
                border-radius: 18px;
                background: linear-gradient(180deg, rgba(31, 45, 80, 0.62), rgba(20, 30, 58, 0.52));
                box-shadow:
                    0 10px 26px rgba(0,0,0,0.18),
                    inset 0 1px 0 rgba(255,255,255,0.04);
            ",

            div {
                style: "
                    color: #9bb0d3;
                    font-size: 0.78rem;
                    text-transform: uppercase;
                    letter-spacing: 0.1em;
                ",
                "{label}"
            }

            strong {
                style: "
                    display: block;
                    margin: 8px 0 4px;
                    color: #82d9ff;
                    font-size: 1.45rem;
                ",
                "{value}"
            }

            span {
                style: "
                    color: #9bb0d3;
                    font-size: 0.86rem;
                    line-height: 1.35;
                ",
                "{detail}"
            }
        }
    }
}