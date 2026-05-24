use dioxus::prelude::*;

use crate::ui::{
    broker_profile_panel::BrokerProfilePanel, dashboard::DashboardPanel, import_panel::ImportPanel,
    llm_lab::LlmLabPanel, search_panel::SearchPanel, settings_panel::SettingsPanel,
    timeline::TimelinePanel,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Workspace {
    Dashboard,
    Timeline,
    Import,
    Search,
    BrokerProfile,
    LlmLab,
    Settings,
}

pub fn AppShell() -> Element {
    let mut workspace = use_signal(|| Workspace::Dashboard);

    rsx! {
        div {
            style: "
                display: grid;
                grid-template-columns: 280px 1fr;
                min-height: 100vh;
                background: #1b1921;
                color: #eee5d2;
                font-family: sans-serif;
            ",

            aside {
                style: "
                    padding: 18px;
                    border-right: 1px solid #4a4058;
                    background: #15131a;
                ",

                h1 {
                    style: "margin: 0; color: #bc8d6b; font-size: 1.35rem;",
                    "Mor X Dossier"
                }

                p {
                    style: "color: #b9aa91; line-height: 1.4;",
                    "Local-first self-brokerage for @MoribundMurdoch."
                }

                nav {
                    style: "display: grid; gap: 8px; margin-top: 20px;",

                    NavButton {
                        label: "Overview",
                        target: Workspace::Dashboard,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::Dashboard)
                    }

                    NavButton {
                        label: "Timeline",
                        target: Workspace::Timeline,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::Timeline)
                    }

                    NavButton {
                        label: "Import",
                        target: Workspace::Import,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::Import)
                    }

                    NavButton {
                        label: "Search",
                        target: Workspace::Search,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::Search)
                    }

                    NavButton {
                        label: "Data Broker Profile",
                        target: Workspace::BrokerProfile,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::BrokerProfile)
                    }

                    NavButton {
                        label: "LLM Mining Lab",
                        target: Workspace::LlmLab,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::LlmLab)
                    }

                    NavButton {
                        label: "Settings",
                        target: Workspace::Settings,
                        current: workspace(),
                        onclick: move |_| workspace.set(Workspace::Settings)
                    }
                }
            }

            main {
                style: "padding: 24px; overflow: auto;",

                match workspace() {
                    Workspace::Dashboard => rsx! { DashboardPanel {} },
                    Workspace::Timeline => rsx! { TimelinePanel {} },
                    Workspace::Import => rsx! { ImportPanel {} },
                    Workspace::Search => rsx! { SearchPanel {} },
                    Workspace::BrokerProfile => rsx! { BrokerProfilePanel {} },
                    Workspace::LlmLab => rsx! { LlmLabPanel {} },
                    Workspace::Settings => rsx! { SettingsPanel {} },
                }
            }
        }
    }
}

#[component]
fn NavButton(
    label: &'static str,
    target: Workspace,
    current: Workspace,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let active = target == current;

    let style = if active {
        "
            text-align: left;
            padding: 10px 12px;
            border: 1px solid #bc8d6b;
            background: #2f2a3a;
            color: #bc8d6b;
            cursor: pointer;
        "
    } else {
        "
            text-align: left;
            padding: 10px 12px;
            border: 1px solid #4a4058;
            background: #24212c;
            color: #eee5d2;
            cursor: pointer;
        "
    };

    rsx! {
        button {
            style: "{style}",
            onclick: move |event| onclick.call(event),
            "{label}"
        }
    }
}
