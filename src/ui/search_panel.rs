use dioxus::prelude::*;

pub fn SearchPanel() -> Element {
    rsx! {
        section {
            h2 { "Search" }
            p { "Build X-style advanced searches and local archive searches." }
        }
    }
}
