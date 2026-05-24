use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        main {
            style: "padding: 24px; font-family: sans-serif;",
            h1 { "MoribundMurdoch X Dossier" }
            p { "The cursed scroll reader has initialized." }
        }
    }
}