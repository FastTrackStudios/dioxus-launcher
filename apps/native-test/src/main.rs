// Blitz compatibility test: progressively add features to find what breaks.
// Known: insert_nodes_before panics when Blitz can't find an anchor node.
// This happens with keyed dynamic lists during reordering.

use dioxus_native::prelude::*;
use launcher_core::QueryEngine;
use providers::DemoProvider;

fn app() -> Element {
    let mut query = use_signal(|| String::new());
    let mut selected = use_signal(|| 0usize);

    let engine = use_signal(|| {
        QueryEngine::builder()
            .max_results(15)
            .provider(Box::new(DemoProvider::new()))
            .build()
    });

    let results = use_memo(move || {
        let q = query.read();
        engine.read().query(&q)
    });

    let count = results.read().len();
    if selected() >= count && count > 0 {
        selected.set(count - 1);
    }

    // Pre-compute display data to keep RSX simple
    let items: Vec<DisplayItem> = results
        .read()
        .iter()
        .enumerate()
        .map(|(idx, item)| DisplayItem {
            label: item.label.clone(),
            sub: item.sub.clone(),
            provider: item.provider.clone(),
            icon: item.icon.chars().next().unwrap_or('#').to_string(),
            tags: item.tags.tags().iter().take(2).map(|t| t.leaf().to_string()).collect(),
            is_selected: idx == selected(),
        })
        .collect();

    rsx! {
        style { {CSS} }
        div { class: "launcher",
            // Search bar
            div { class: "search",
                span { class: "search-icon", "\u{2315}" }
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "Search...",
                    autofocus: true,
                    value: "{query}",
                    oninput: move |evt| {
                        query.set(evt.value());
                        selected.set(0);
                    },
                    onkeydown: move |evt| {
                        match evt.key() {
                            Key::ArrowDown | Key::Tab => {
                                evt.prevent_default();
                                if count > 0 { selected.set((selected() + 1) % count); }
                            }
                            Key::ArrowUp => {
                                evt.prevent_default();
                                if count > 0 { selected.set(selected().checked_sub(1).unwrap_or(count - 1)); }
                            }
                            Key::PageDown => {
                                evt.prevent_default();
                                if count > 0 { selected.set((selected() + 10).min(count - 1)); }
                            }
                            Key::PageUp => {
                                evt.prevent_default();
                                selected.set(selected().saturating_sub(10));
                            }
                            Key::Home => { evt.prevent_default(); selected.set(0); }
                            Key::End => { evt.prevent_default(); if count > 0 { selected.set(count - 1); } }
                            Key::Escape => { query.set(String::new()); selected.set(0); }
                            _ => {}
                        }
                    },
                }
                span { class: "count", "{count} results" }
            }

            // Results
            div { class: "results",
                for item in items {
                    div {
                        class: if item.is_selected { "item selected" } else { "item" },
                        div { class: "icon-wrap", "{item.icon}" }
                        div { class: "body",
                            div { class: "label", "{item.label}" }
                            div { class: "sub", "{item.sub}" }
                            if !item.tags.is_empty() {
                                div { class: "tags",
                                    for tag in item.tags {
                                        span { class: "tag", "{tag}" }
                                    }
                                }
                            }
                        }
                        span { class: "badge", "{item.provider}" }
                    }
                }
            }

            // Status bar
            div { class: "status",
                span {
                    if count > 0 { "{selected() + 1}/{count}" } else { "No results" }
                }
                span { "\u{2191}\u{2193} navigate \u{00B7} Esc clear" }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct DisplayItem {
    label: String,
    sub: String,
    provider: String,
    icon: String,
    tags: Vec<String>,
    is_selected: bool,
}

const CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
body { background: #1e1e2e; color: #cdd6f4; font-family: system-ui, sans-serif; font-size: 14px; line-height: 1.5; }

.launcher { display: flex; flex-direction: column; height: 100vh; }

.search {
    display: flex; align-items: center; padding: 12px 16px;
    border-bottom: 1px solid #45475a; gap: 10px;
}
.search-icon { color: #89b4fa; font-size: 18px; }
.search-input {
    flex: 1; background: transparent; border: none; outline: none;
    color: #cdd6f4; font-size: 16px; font-family: inherit; caret-color: #89b4fa;
}
.search-input::placeholder { color: #6c7086; }
.count { color: #6c7086; font-size: 12px; }

.results { flex: 1; overflow-y: auto; padding: 4px 8px; }

.item {
    display: flex; align-items: center; padding: 8px 12px;
    border-radius: 6px; gap: 12px;
}
.item:hover { background: #45475a; }
.item.selected { background: #585b70; }

.icon-wrap {
    width: 36px; height: 36px; display: flex; align-items: center;
    justify-content: center; border-radius: 6px; background: #313244;
    color: #89b4fa; font-size: 16px; font-weight: 600; flex-shrink: 0;
}

.body { flex: 1; min-width: 0; }
.label { font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.sub { font-size: 12px; color: #a6adc8; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.tags { display: flex; gap: 4px; margin-top: 2px; }
.tag {
    font-size: 10px; padding: 0 6px; border-radius: 9999px;
    background: #313244; color: #a6adc8; border: 1px solid #45475a;
}

.badge {
    font-size: 11px; color: #6c7086; padding: 2px 6px;
    background: #313244; border-radius: 4px; flex-shrink: 0;
}

.status {
    display: flex; justify-content: space-between; padding: 6px 16px;
    border-top: 1px solid #45475a; font-size: 12px; color: #6c7086;
    background: #181825;
}
"#;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("error")
        .init();

    dioxus_native::launch(app);
}
