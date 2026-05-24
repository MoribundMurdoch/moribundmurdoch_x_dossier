use dioxus::prelude::*;

pub fn ImportPanel() -> Element {
    rsx! {
        section {
            h2 { "Import" }
            p { "Start with an X archive ZIP, then use the API for incremental sync." }

            div {
                style: "display: flex; gap: 10px; flex-wrap: wrap; margin-top: 16px;",

                button { "Import X Archive ZIP" }
                button { "Sync Latest via API" }
                button { "Reconcile Sources" }
            }
        }
    }
}
