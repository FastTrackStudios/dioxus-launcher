use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DisplayItem {
    pub label: String,
    pub sub: String,
    pub provider: String,
    pub icon: String,
    pub icon_path: String,
    pub tags: Vec<String>,
    pub is_selected: bool,
    pub is_favorite: bool,
    pub is_new: bool,
    pub rating: u8,
    pub note: String,
    pub index: usize,
    pub action_count: usize,
    pub item_id: String,
    pub label_segments: Vec<TextSegment>,
}

#[derive(Clone, PartialEq)]
pub struct TextSegment {
    pub text: String,
    pub highlighted: bool,
}

impl DisplayItem {
    pub fn from_item(item: &launcher_core::Item, index: usize, selected: bool) -> Self {
        let is_icon_path = item.icon.starts_with('/');
        let icon_path = if is_icon_path { item.icon.clone() } else { String::new() };
        let icon_initial = if is_icon_path || item.icon.is_empty() {
            item.label.chars().next().unwrap_or('#').to_uppercase().next().unwrap_or('#').to_string()
        } else {
            item.icon.chars().next().unwrap_or('#').to_uppercase().next().unwrap_or('#').to_string()
        };

        Self {
            label: item.label.clone(),
            sub: item.sub.clone(),
            provider: item.provider.clone(),
            icon: icon_initial,
            icon_path,
            tags: item.tags.tags().iter().take(3).map(|t| t.leaf().to_string()).collect(),
            is_selected: selected,
            is_favorite: item.pinned,
            is_new: false,
            rating: 0,
            note: String::new(),
            index,
            action_count: item.actions.len(),
            item_id: item.id.clone(),
            label_segments: highlight_segments(&item.label, &item.match_positions),
        }
    }
}

#[component]
pub fn ResultItem(item: DisplayItem, on_activate: EventHandler<usize>) -> Element {
    let base = "flex items-center px-3 py-2 rounded-md cursor-pointer gap-3 min-h-12";
    let class = if item.is_selected {
        format!("{base} bg-ctp-surface2")
    } else {
        format!("{base} hover:bg-ctp-surface1")
    };

    let idx = item.index;
    let shortcut = if item.index < 9 { format!("\u{2318}{}", item.index + 1) } else { String::new() };
    let has_tags = !item.tags.is_empty();
    let has_extra_actions = item.action_count > 1;
    let extra_action_label = format!("+{}", item.action_count - 1);
    let has_shortcut = !shortcut.is_empty();
    let fav_icon = if item.is_favorite { "\u{2605}" } else { "" };
    let has_fav = item.is_favorite;
    let has_rating = item.rating > 0;
    let rating_str = "\u{2605}".repeat(item.rating as usize);
    let is_new = item.is_new;
    let has_note = !item.note.is_empty();
    let has_icon_image = !item.icon_path.is_empty();

    rsx! {
        div {
            class: "{class}",
            onclick: move |_| on_activate.call(idx),

            if has_icon_image {
                div { class: "w-9 h-9 flex items-center justify-center rounded-md bg-ctp-surface0 shrink-0",
                    img {
                        class: "w-7 h-7 object-contain rounded",
                        src: "{item.icon_path}",
                    }
                }
            } else {
                div { class: "w-9 h-9 flex items-center justify-center rounded-md bg-ctp-surface0 text-ctp-blue font-semibold shrink-0",
                    "{item.icon}"
                }
            }

            div { class: "flex-1 min-w-0 flex flex-col gap-px",
                div { class: "text-sm font-medium text-ctp-text truncate",
                    for seg in &item.label_segments {
                        if seg.highlighted {
                            span { class: "text-ctp-yellow font-semibold", "{seg.text}" }
                        } else {
                            span { "{seg.text}" }
                        }
                    }
                }
                if !item.sub.is_empty() {
                    div { class: "text-xs text-ctp-subtext0 truncate", "{item.sub}" }
                }
                if has_tags {
                    div { class: "flex gap-1 flex-wrap mt-0.5",
                        for tag in &item.tags {
                            span { class: "text-[10px] px-1.5 rounded-full bg-ctp-surface0 text-ctp-subtext0 border border-ctp-surface1",
                                "{tag}"
                            }
                        }
                    }
                }
                if has_note {
                    div { class: "text-[11px] text-ctp-overlay0 italic truncate mt-0.5",
                        "\u{1F4DD} {item.note}"
                    }
                }
            }

            div { class: "flex items-center gap-1.5 shrink-0",
                if is_new {
                    span { class: "text-[9px] px-1.5 py-px rounded bg-ctp-green text-ctp-base font-semibold", "NEW" }
                }
                if has_rating {
                    span { class: "text-[11px] text-ctp-yellow", "{rating_str}" }
                }
                if has_fav {
                    span { class: "text-ctp-yellow text-sm", "{fav_icon}" }
                }
                span { class: "text-[11px] text-ctp-overlay0 px-1.5 py-px bg-ctp-surface0 rounded", "{item.provider}" }
                if has_extra_actions {
                    span { class: "text-[11px] text-ctp-overlay0 opacity-60", "{extra_action_label}" }
                }
                if has_shortcut {
                    span { class: "text-[11px] text-ctp-overlay0 font-mono px-1 py-px bg-ctp-surface0 border border-ctp-surface1 rounded",
                        "{shortcut}"
                    }
                }
            }
        }
    }
}

pub fn highlight_segments(text: &str, positions: &[u32]) -> Vec<TextSegment> {
    if positions.is_empty() {
        return vec![TextSegment { text: text.to_string(), highlighted: false }];
    }
    let mut segments = Vec::new();
    let mut last = 0;
    let chars: Vec<char> = text.chars().collect();
    for &pos in positions {
        let pos = pos as usize;
        if pos >= chars.len() { continue; }
        if pos > last {
            segments.push(TextSegment { text: chars[last..pos].iter().collect(), highlighted: false });
        }
        segments.push(TextSegment { text: chars[pos..=pos].iter().collect(), highlighted: true });
        last = pos + 1;
    }
    if last < chars.len() {
        segments.push(TextSegment { text: chars[last..].iter().collect(), highlighted: false });
    }
    segments
}
