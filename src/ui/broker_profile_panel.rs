use dioxus::prelude::*;

pub fn BrokerProfilePanel() -> Element {
    rsx! {
        section {
            h2 { "Data Broker Profile" }
            p { "The app will show what a nosy ad-tech gremlin might infer, with evidence and confidence scores." }
        }
    }
}
