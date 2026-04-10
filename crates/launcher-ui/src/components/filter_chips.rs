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

#[derive(Clone, PartialEq, Debug)]
pub enum FilterAction {
    Clear,
    ToggleInclude(String),
    ToggleExclude(String),
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
        div { class: "flex gap-1 px-4 py-1 border-b border-ctp-surface1 overflow-x-auto shrink-0 bg-ctp-base",
            if has_active {
                span {
                    class: "flex items-center gap-0.5 px-2.5 py-0.5 rounded-full bg-ctp-surface0 text-ctp-subtext0 text-[11px] cursor-pointer hover:bg-ctp-surface1 whitespace-nowrap",
                    onclick: move |_| on_action.call(FilterAction::Clear),
                    "\u{2715} Clear"
                }
            }

            for chip in &chips {
                {
                    let base = "flex items-center gap-0.5 px-2.5 py-0.5 rounded-full text-[11px] cursor-pointer whitespace-nowrap border";
                    let class = if chip.is_active {
                        format!("{base} bg-ctp-blue text-ctp-base border-ctp-blue")
                    } else if chip.is_excluded {
                        format!("{base} bg-ctp-red text-ctp-base border-ctp-red opacity-85")
                    } else {
                        format!("{base} bg-ctp-surface0 text-ctp-subtext0 border-transparent hover:bg-ctp-surface1 hover:text-ctp-text")
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
                            onclick: move |evt: MouseEvent| {
                                let mods = evt.modifiers();
                                if mods.ctrl() {
                                    on_action.call(FilterAction::Remove(id_rem.clone()));
                                } else if mods.alt() || mods.shift() {
                                    on_action.call(FilterAction::ToggleExclude(id_exc.clone()));
                                } else if is_active || is_excluded {
                                    on_action.call(FilterAction::Remove(id_inc.clone()));
                                } else {
                                    on_action.call(FilterAction::ToggleInclude(id_inc.clone()));
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
