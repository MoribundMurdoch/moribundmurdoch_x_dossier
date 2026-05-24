use dioxus::prelude::*;

pub fn LlmLabPanel() -> Element {
    rsx! {
        section {
            h2 { "LLM Mining Lab" }
            p { "Generate JSONL, Markdown, summaries, topic packs, and context bundles." }
        }
    }
}
