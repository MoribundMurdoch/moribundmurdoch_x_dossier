use dioxus::prelude::*;

pub fn SettingsPanel() -> Element {
    rsx! {
        section {
            h2 { "Settings" }
            p { "API tokens, archive paths, local database location, and redaction rules." }
        }
    }
}
