use dioxus::prelude::*;

pub fn TimelinePanel() -> Element {
    rsx! {
        section {
            h2 { "Timeline" }
            p { "No records imported yet. Feed the machine an archive ZIP." }
        }
    }
}
