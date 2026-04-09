use crate::prelude::*;

/// Inline Quick Tag overlay for assigning tags to the selected item.
/// Shows a text input that filters available tags, Enter to assign.
#[component]
pub fn QuickTag(
    item_id: String,
    item_label: String,
    current_tags: Vec<String>,
    available_tags: Vec<QuickTagOption>,
    on_add: EventHandler<String>,
    on_remove: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    let mut tag_query = use_signal(|| String::new());

    let filtered: Vec<&QuickTagOption> = available_tags
        .iter()
        .filter(|t| {
            if tag_query.read().is_empty() {
                true
            } else {
                let q = tag_query.read().to_lowercase();
                t.label.to_lowercase().contains(&q) || t.path.to_lowercase().contains(&q)
            }
        })
        .take(15)
        .collect();

    rsx! {
        div {
            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 100;",
            onclick: move |_| on_close.call(()),

            div {
                style: "background: var(--bg-primary); border: 1px solid var(--border); border-radius: var(--radius-lg); padding: 16px; width: 400px; max-height: 400px; display: flex; flex-direction: column; gap: 8px; box-shadow: var(--shadow-lg);",
                onclick: move |evt| evt.stop_propagation(),

                div {
                    style: "font-size: 14px; font-weight: 600; color: var(--text-primary);",
                    "Tag: {item_label}"
                }

                // Current tags
                if !current_tags.is_empty() {
                    div {
                        style: "display: flex; gap: 4px; flex-wrap: wrap;",
                        for tag in &current_tags {
                            {
                                let tag_rm = tag.clone();
                                rsx! {
                                    span {
                                        style: "font-size: 11px; padding: 2px 8px; border-radius: 9999px; background: var(--accent); color: var(--text-inverse); cursor: pointer;",
                                        onclick: move |_| on_remove.call(tag_rm.clone()),
                                        "{tag} \u{2715}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Search input
                input {
                    style: "background: var(--bg-input); border: 1px solid var(--border); border-radius: var(--radius-md); padding: 6px 10px; color: var(--text-primary); font-size: 13px; outline: none; width: 100%;",
                    r#type: "text",
                    placeholder: "Search tags...",
                    autofocus: true,
                    value: "{tag_query}",
                    oninput: move |evt| tag_query.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            on_close.call(());
                        }
                    },
                }

                // Available tags list
                div {
                    style: "overflow-y: auto; max-height: 250px; display: flex; flex-direction: column; gap: 2px;",
                    for opt in filtered {
                        {
                            let path = opt.path.clone();
                            let is_assigned = current_tags.contains(&opt.path);
                            let bg = if is_assigned { "var(--bg-selected)" } else { "transparent" };
                            let color_style = if !opt.color.is_empty() {
                                format!("border-left: 3px solid {};", opt.color)
                            } else {
                                String::new()
                            };
                            rsx! {
                                div {
                                    style: "padding: 4px 8px; border-radius: var(--radius-sm); cursor: pointer; font-size: 12px; color: var(--text-secondary); background: {bg}; {color_style}",
                                    onclick: move |_| on_add.call(path.clone()),
                                    span { style: "font-weight: 500; color: var(--text-primary);", "{opt.label}" }
                                    span { style: "margin-left: 8px; color: var(--text-muted); font-size: 10px;", "{opt.path}" }
                                    if is_assigned {
                                        span { style: "float: right; color: var(--accent);", "\u{2713}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct QuickTagOption {
    pub path: String,
    pub label: String,
    pub color: String,
}
