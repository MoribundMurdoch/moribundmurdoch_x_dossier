use dioxus::prelude::*;

pub fn DashboardPanel() -> Element {
    rsx! {
        section {
            h2 { "MoribundMurdoch X Dossier" }

            p {
                "A public-friendly archive viewer for MoribundMurdoch's X/Twitter history, \
                dressed up as a parody data-broker dashboard and LLM mining exhibit."
            }

            div {
                style: "
                    margin-top: 18px;
                    border: 1px solid #4a4058;
                    background: #24212c;
                    padding: 14px;
                ",

                h3 { "Purpose" }

                ul {
                    li { "Make old posts easier to browse than they are on X." }
                    li { "Show what a data broker might try to infer from public activity." }
                    li { "Mock the hunger for human-generated content by LLM companies." }
                    li { "Preserve context so machines and humans misread less confidently." }
                }
            }
        }
    }
}
