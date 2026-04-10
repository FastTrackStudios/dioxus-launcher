use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActionInfo {
    pub name: String,
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
        div { class: "flex items-center px-4 py-1 border-t border-ctp-surface1 h-9 bg-ctp-mantle gap-4 shrink-0",
            div { class: "flex items-center gap-2 flex-1 min-w-0",
                for action in &actions {
                    span { class: "flex items-center gap-0.5 text-[11px] text-ctp-overlay0",
                        span { class: "font-mono text-[11px] px-1 py-px bg-ctp-surface0 border border-ctp-surface1 rounded text-ctp-overlay0",
                            "{action.modifier_label}"
                        }
                        " {action.name}"
                    }
                }
            }

            div { class: "flex items-center gap-1.5 shrink-0",
                if has_results {
                    span { class: "text-[11px] text-ctp-overlay0", "{counter}" }
                    span { class: "w-px h-4 bg-ctp-surface1" }
                }
                span { class: "text-[11px] text-ctp-overlay0",
                    span { class: "font-mono px-1 py-px bg-ctp-surface0 border border-ctp-surface1 rounded mr-0.5", "\u{2191}\u{2193}" }
                    " nav"
                }
                span { class: "text-[11px] text-ctp-overlay0",
                    span { class: "font-mono px-1 py-px bg-ctp-surface0 border border-ctp-surface1 rounded mr-0.5", "Esc" }
                    " clear"
                }
                span { class: "text-[11px] text-ctp-overlay0",
                    span { class: "font-mono px-1 py-px bg-ctp-surface0 border border-ctp-surface1 rounded mr-0.5", "Tab" }
                    " {mode_label}"
                }
            }
        }
    }
}
