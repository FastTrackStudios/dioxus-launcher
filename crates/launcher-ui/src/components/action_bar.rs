use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActionInfo {
    pub name: String,
    /// Modifier label for the keyboard shortcut (e.g. "⏎", "⇧⏎", "^⏎").
    pub modifier_label: String,
}

#[component]
pub fn ActionBar(
    actions: Vec<ActionInfo>,
    result_count: usize,
    selected_index: usize,
    mode_label: String,
) -> Element {
    let has_results = result_count > 0;
    let counter = format!("{}/{}", selected_index + 1, result_count);

    rsx! {
        div { class: "action-bar",
            div { class: "action-bar-left",
                for action in &actions {
                    span { class: "action-btn",
                        span { class: "action-kbd", "{action.modifier_label}" }
                        " {action.name}"
                    }
                }
            }

            div { class: "action-bar-right",
                if has_results {
                    span { class: "action-btn", "{counter}" }
                    span { class: "action-divider" }
                }

                span { class: "action-btn",
                    span { class: "action-kbd", "\u{2191}\u{2193}" }
                    " nav"
                }
                span { class: "action-btn",
                    span { class: "action-kbd", "Esc" }
                    " clear"
                }
                span { class: "action-btn",
                    span { class: "action-kbd", "Tab" }
                    " {mode_label}"
                }
            }
        }
    }
}
