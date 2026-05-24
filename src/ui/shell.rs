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
    Search,
    BrokerProfile,
    LlmLab,
    Maintainer,
    Settings,
}

impl Workspace {
    fn title(self) -> &'static str {
        match self {
            Workspace::Dashboard => "Overview",
            Workspace::Timeline => "Timeline",
            Workspace::Search => "Search",
            Workspace::BrokerProfile => "Data Broker Mirror",
            Workspace::LlmLab => "LLM Mining Exhibit",
            Workspace::Maintainer => "Maintainer",
            Workspace::Settings => "Settings",
        }
    }

    fn system_hint(self) -> &'static str {
        match self {
            Workspace::Dashboard => "Public archive // human residue terminal",
            Workspace::Timeline => "Chronological signal stream",
            Workspace::Search => "Index scan // context retrieval",
            Workspace::BrokerProfile => "Satirical inference engine",
            Workspace::LlmLab => "Machine appetite exhibit",
            Workspace::Maintainer => "Dataset forge // private controls",
            Workspace::Settings => "Local configuration",
        }
    }
}

pub fn AppShell() -> Element {
    let mut workspace = use_signal(|| Workspace::Dashboard);

    rsx! {
        div {
            style: "
                min-height: 100vh;
                display: grid;
                grid-template-columns: 300px 1fr;
                color: #dce8ff;
                font-family: 'Segoe UI', Inter, Roboto, sans-serif;
                letter-spacing: 0.015em;
                background:
                    radial-gradient(circle at 16% 16%, rgba(104, 132, 255, 0.18), transparent 28%),
                    radial-gradient(circle at 84% 12%, rgba(127, 217, 255, 0.16), transparent 30%),
                    radial-gradient(circle at 50% 92%, rgba(193, 168, 255, 0.12), transparent 34%),
                    linear-gradient(145deg, #080a13, #10172a 48%, #18213b);
            ",

            aside {
                style: "
                    position: relative;
                    overflow: hidden;
                    padding: 22px;
                    border-right: 1px solid rgba(135, 178, 255, 0.24);
                    background:
                        linear-gradient(180deg, rgba(10, 16, 31, 0.96), rgba(16, 25, 48, 0.86));
                    box-shadow:
                        inset -1px 0 0 rgba(255,255,255,0.03),
                        12px 0 40px rgba(0,0,0,0.18);
                ",

                div {
                    style: "
                        position: absolute;
                        width: 220px;
                        height: 220px;
                        top: -80px;
                        left: -80px;
                        border-radius: 999px;
                        background: rgba(130, 217, 255, 0.12);
                        filter: blur(18px);
                        pointer-events: none;
                    "
                }

                div {
                    style: "position: relative; z-index: 1;",

                    div {
                        style: "
                            margin-bottom: 18px;
                            padding: 14px;
                            border: 1px solid rgba(135, 178, 255, 0.22);
                            border-radius: 20px;
                            background: linear-gradient(180deg, rgba(31, 45, 80, 0.58), rgba(20, 30, 58, 0.38));
                            box-shadow: inset 0 1px 0 rgba(255,255,255,0.05);
                        ",

                        h1 {
                            style: "
                                margin: 0;
                                color: #82d9ff;
                                font-size: 1.35rem;
                                font-weight: 650;
                                text-shadow: 0 0 18px rgba(130, 217, 255, 0.35);
                            ",
                            "Mor X Dossier"
                        }

                        p {
                            style: "
                                margin: 8px 0 0;
                                color: #9bb0d3;
                                line-height: 1.45;
                                font-size: 0.88rem;
                            ",
                            "Public archive viewer, data-broker parody, and LLM mining exhibit."
                        }
                    }

                    div {
                        style: "
                            display: grid;
                            gap: 6px;
                            margin-bottom: 18px;
                            padding: 12px;
                            border: 1px solid rgba(142, 150, 255, 0.28);
                            border-radius: 18px;
                            background: rgba(10, 16, 31, 0.48);
                        ",

                        span {
                            style: "color: #c1a8ff; font-size: 0.78rem; text-transform: uppercase; letter-spacing: 0.12em;",
                            "Archive Signal"
                        }

                        strong {
                            style: "color: #f4f8ff; font-size: 0.95rem;",
                            "Context Integrity: Fragile"
                        }

                        span {
                            style: "color: #9bb0d3; font-size: 0.82rem; line-height: 1.35;",
                            "Human-generated whatnot detected. Mine responsibly."
                        }
                    }

                    nav {
                        style: "display: grid; gap: 10px;",

                        NavButton {
                            label: "Overview",
                            detail: "project home",
                            target: Workspace::Dashboard,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::Dashboard)
                        }

                        NavButton {
                            label: "Timeline",
                            detail: "post stream",
                            target: Workspace::Timeline,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::Timeline)
                        }

                        NavButton {
                            label: "Search",
                            detail: "archive scan",
                            target: Workspace::Search,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::Search)
                        }

                        NavButton {
                            label: "Data Broker Mirror",
                            detail: "fake creepy inferences",
                            target: Workspace::BrokerProfile,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::BrokerProfile)
                        }

                        NavButton {
                            label: "LLM Mining Exhibit",
                            detail: "context warnings",
                            target: Workspace::LlmLab,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::LlmLab)
                        }

                        NavButton {
                            label: "Maintainer",
                            detail: "private dataset tools",
                            target: Workspace::Maintainer,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::Maintainer)
                        }

                        NavButton {
                            label: "Settings",
                            detail: "local controls",
                            target: Workspace::Settings,
                            current: workspace(),
                            onclick: move |_| workspace.set(Workspace::Settings)
                        }
                    }
                }
            }

            main {
                style: "
                    position: relative;
                    overflow: auto;
                    padding: 26px;
                ",

                div {
                    style: "
                        position: fixed;
                        inset: 0;
                        pointer-events: none;
                        background:
                            linear-gradient(to bottom, rgba(255,255,255,0.035), transparent 18%),
                            repeating-linear-gradient(
                                to bottom,
                                rgba(255,255,255,0.018) 0,
                                rgba(255,255,255,0.018) 1px,
                                transparent 1px,
                                transparent 24px
                            );
                        opacity: 0.38;
                    "
                }

                div {
                    style: "
                        position: relative;
                        z-index: 1;
                        max-width: 1180px;
                    ",

                    header {
                        style: "
                            margin-bottom: 18px;
                            padding: 16px 18px;
                            border: 1px solid rgba(135, 178, 255, 0.22);
                            border-radius: 22px;
                            background: linear-gradient(180deg, rgba(20, 31, 58, 0.62), rgba(16, 25, 47, 0.46));
                            box-shadow:
                                0 18px 40px rgba(0,0,0,0.22),
                                inset 0 1px 0 rgba(255,255,255,0.05);
                        ",

                        div {
                            style: "color: #82d9ff; font-size: 0.78rem; text-transform: uppercase; letter-spacing: 0.14em;",
                            "Moribund Public Dataset Terminal"
                        }

                        h2 {
                            style: "margin: 6px 0 4px; color: #f4f8ff; font-size: 1.45rem;",
                            "{workspace().title()}"
                        }

                        p {
                            style: "margin: 0; color: #9bb0d3; font-size: 0.92rem;",
                            "{workspace().system_hint()}"
                        }
                    }

                    match workspace() {
                        Workspace::Dashboard => rsx! { DashboardPanel {} },
                        Workspace::Timeline => rsx! { TimelinePanel {} },
                        Workspace::Search => rsx! { SearchPanel {} },
                        Workspace::BrokerProfile => rsx! { BrokerProfilePanel {} },
                        Workspace::LlmLab => rsx! { LlmLabPanel {} },
                        Workspace::Maintainer => rsx! { ImportPanel {} },
                        Workspace::Settings => rsx! { SettingsPanel {} },
                    }
                }
            }
        }
    }
}

#[component]
fn NavButton(
    label: &'static str,
    detail: &'static str,
    target: Workspace,
    current: Workspace,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let active = target == current;

    let style = if active {
        "
            text-align: left;
            padding: 12px 14px;
            border: 1px solid rgba(142, 150, 255, 0.72);
            border-radius: 16px;
            background: linear-gradient(180deg, rgba(49, 64, 117, 0.94), rgba(27, 42, 84, 0.86));
            color: #f4f8ff;
            cursor: pointer;
            box-shadow:
                0 0 0 1px rgba(130, 217, 255, 0.16),
                0 0 24px rgba(143, 150, 255, 0.22),
                inset 0 1px 0 rgba(255,255,255,0.08);
        "
    } else {
        "
            text-align: left;
            padding: 12px 14px;
            border: 1px solid rgba(135, 178, 255, 0.22);
            border-radius: 16px;
            background: linear-gradient(180deg, rgba(27, 40, 72, 0.72), rgba(19, 29, 54, 0.58));
            color: #dce8ff;
            cursor: pointer;
            box-shadow:
                0 8px 24px rgba(0,0,0,0.16),
                inset 0 1px 0 rgba(255,255,255,0.04);
        "
    };

    rsx! {
        button {
            style: "{style}",
            onclick: move |event| onclick.call(event),

            span {
                style: "display: block; font-size: 0.95rem; font-weight: 650;",
                "{label}"
            }

            span {
                style: "display: block; margin-top: 3px; color: #9bb0d3; font-size: 0.76rem;",
                "{detail}"
            }
        }
    }
}
