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
        div { class: "search-container",
            span { class: "search-icon", "\u{2315}" }

            if has_tag {
                span {
                    class: "filter-chip active",
                    style: "margin-right: 4px; cursor: pointer;",
                    onclick: move |_| on_clear_tag.call(()),
                    "#{active_tag_label} \u{2715}"
                }
            }

            input {
                class: "search-input",
                r#type: "text",
                placeholder: "{placeholder}",
                autofocus: true,
                value: "{value}",
                oninput: move |evt| on_input.call(evt.value()),
                onkeydown: move |evt| on_key.call(evt),
            }

            div { class: "search-meta",
                if show_count {
                    span { class: "search-count", "{result_count} results" }
                }
                button {
                    class: "search-mode-toggle",
                    onclick: move |_| on_toggle_mode.call(()),
                    "{mode.label()}"
                }
            }
        }
    }
}
