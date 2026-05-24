use dioxus::prelude::*;

use crate::ui::shell::AppShell;

pub fn App() -> Element {
    rsx! {
        AppShell {}
    }
}
