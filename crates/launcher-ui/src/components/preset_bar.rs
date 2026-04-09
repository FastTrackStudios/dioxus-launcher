use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PresetChip {
    pub name: String,
}

#[component]
pub fn PresetBar(
    presets: Vec<PresetChip>,
    on_load: EventHandler<String>,
    on_save: EventHandler<()>,
    on_delete: EventHandler<String>,
    show_save: bool,
) -> Element {
    if presets.is_empty() && !show_save {
        return rsx! {};
    }

    rsx! {
        div {
            style: "display: flex; gap: 4px; padding: 2px 16px; border-bottom: 1px solid var(--border); overflow-x: auto; align-items: center; flex-shrink: 0; background: var(--bg-tertiary);",

            span {
                style: "font-size: 10px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; flex-shrink: 0; margin-right: 4px;",
                "Presets"
            }

            for preset in &presets {
                {
                    let name = preset.name.clone();
                    let del_name = preset.name.clone();
                    rsx! {
                        span {
                            style: "display: flex; align-items: center; gap: 2px; padding: 2px 8px; border-radius: 9999px; background: var(--bg-secondary); color: var(--text-secondary); font-size: 11px; cursor: pointer; border: 1px solid var(--border); white-space: nowrap;",
                            onclick: move |_| on_load.call(name.clone()),
                            "{preset.name}"
                            span {
                                style: "margin-left: 4px; opacity: 0.5; font-size: 9px; cursor: pointer;",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    on_delete.call(del_name.clone());
                                },
                                "\u{2715}"
                            }
                        }
                    }
                }
            }

            if show_save {
                span {
                    style: "padding: 2px 8px; border-radius: 9999px; background: var(--accent-dim); color: var(--accent); font-size: 11px; cursor: pointer; border: 1px solid var(--accent); white-space: nowrap; opacity: 0.8;",
                    onclick: move |_| on_save.call(()),
                    "+ Save preset"
                }
            }
        }
    }
}
