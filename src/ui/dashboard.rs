use dioxus::prelude::*;

pub fn DashboardPanel() -> Element {
    rsx! {
        section {
            h2 { "Overview" }
            p { "Posts, replies, reposts, likes, links, hashtags, and the whole cursed behavioral pantry." }
        }
    }
}
