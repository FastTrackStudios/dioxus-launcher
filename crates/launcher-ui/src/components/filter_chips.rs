use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ChipItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub is_active: bool,
    pub is_excluded: bool,
    pub color: String,
}

/// What kind of filter action was requested.
#[derive(Clone, PartialEq, Debug)]
pub enum FilterAction {
    /// Clear all filters.
    Clear,
    /// Toggle include (click).
    ToggleInclude(String),
    /// Toggle exclude (Alt+click or Shift+click).
    ToggleExclude(String),
    /// Remove from all filter lists (Ctrl+click).
    Remove(String),
}

#[component]
pub fn FilterChips(
    chips: Vec<ChipItem>,
    has_active: bool,
    on_action: EventHandler<FilterAction>,
) -> Element {
    if chips.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "filter-chips",
            if has_active {
                span {
                    class: "filter-chip",
                    onclick: move |_| on_action.call(FilterAction::Clear),
                    "\u{2715} Clear"
                }
            }

            for chip in &chips {
                {
                    let class = if chip.is_active {
                        "filter-chip active"
                    } else if chip.is_excluded {
                        "filter-chip excluded"
                    } else {
                        "filter-chip"
                    };

                    let style = if !chip.color.is_empty() && (chip.is_active || chip.is_excluded) {
                        format!("border-color: {}; color: {};", chip.color, chip.color)
                    } else if !chip.color.is_empty() {
                        format!("border-color: {}33;", chip.color)
                    } else {
                        String::new()
                    };

                    let prefix = if chip.is_excluded { "-" } else if chip.is_active { "+" } else { "" };
                    let id_inc = chip.id.clone();
                    let id_exc = chip.id.clone();
                    let id_rem = chip.id.clone();
                    let is_active = chip.is_active;
                    let is_excluded = chip.is_excluded;

                    rsx! {
                        span {
                            class: "{class}",
                            style: "{style}",
                            onclick: move |evt: MouseEvent| {
                                let mods = evt.modifiers();
                                if mods.ctrl() {
                                    // Ctrl+click: remove from all
                                    on_action.call(FilterAction::Remove(id_rem.clone()));
                                } else if mods.alt() || mods.shift() {
                                    // Alt+click or Shift+click: toggle exclude
                                    on_action.call(FilterAction::ToggleExclude(id_exc.clone()));
                                } else {
                                    // Plain click: toggle include (or clear if already active)
                                    if is_active {
                                        on_action.call(FilterAction::Remove(id_inc.clone()));
                                    } else if is_excluded {
                                        on_action.call(FilterAction::Remove(id_inc.clone()));
                                    } else {
                                        on_action.call(FilterAction::ToggleInclude(id_inc.clone()));
                                    }
                                }
                            },
                            "{prefix}{chip.label}"
                        }
                    }
                }
            }
        }
    }
}
