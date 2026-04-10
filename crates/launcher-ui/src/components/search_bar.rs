use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LauncherMode {
    Full,
    Palette,
}

impl LauncherMode {
    pub fn label(&self) -> &str {
        match self {
            LauncherMode::Full => "Full",
            LauncherMode::Palette => "Palette",
        }
    }
}

#[component]
pub fn SearchBar(
    value: String,
    on_input: EventHandler<String>,
    on_key: EventHandler<KeyboardEvent>,
    result_count: usize,
    mode: LauncherMode,
    on_toggle_mode: EventHandler<()>,
    active_tag_label: String,
    on_clear_tag: EventHandler<()>,
) -> Element {
    let has_tag = !active_tag_label.is_empty();
    let show_count = !value.is_empty() || has_tag;
    let placeholder = if has_tag { "Filter..." } else { "Search..." };

    rsx! {
        div { class: "flex items-center px-4 py-3 border-b border-ctp-surface1 gap-2 shrink-0 bg-ctp-base",
            span { class: "text-ctp-blue text-lg shrink-0 opacity-80", "\u{2315}" }

            if has_tag {
                span {
                    class: "text-xs px-2 py-0.5 rounded-full bg-ctp-blue text-ctp-base cursor-pointer mr-1 shrink-0",
                    onclick: move |_| on_clear_tag.call(()),
                    "#{active_tag_label} \u{2715}"
                }
            }

            input {
                class: "flex-1 bg-transparent border-none outline-none text-ctp-text text-base caret-ctp-blue",
                r#type: "text",
                placeholder: "{placeholder}",
                autofocus: true,
                value: "{value}",
                oninput: move |evt| on_input.call(evt.value()),
                onkeydown: move |evt| on_key.call(evt),
            }

            div { class: "flex items-center gap-2 shrink-0",
                if show_count {
                    span { class: "text-ctp-overlay0 text-xs", "{result_count} results" }
                }
                button {
                    class: "px-2 py-0.5 rounded text-xs bg-ctp-surface0 text-ctp-overlay0 border border-ctp-surface1 cursor-pointer hover:text-ctp-text hover:border-ctp-blue",
                    onclick: move |_| on_toggle_mode.call(()),
                    "{mode.label()}"
                }
            }
        }
    }
}
