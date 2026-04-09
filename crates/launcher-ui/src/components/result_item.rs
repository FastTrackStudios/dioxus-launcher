use crate::prelude::*;

/// Pre-computed display data for a result item.
/// Keeping this as a plain struct (not an Item ref) avoids complex
/// lifetime/borrow issues in RSX and works well with Blitz.
#[derive(Clone, PartialEq)]
pub struct DisplayItem {
    pub label: String,
    pub sub: String,
    pub provider: String,
    /// Icon initial letter (fallback).
    pub icon: String,
    /// Resolved icon file path (PNG/SVG) — empty if none.
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

/// Convert an Item + index + selected into a DisplayItem.
impl DisplayItem {
    pub fn from_item(item: &launcher_core::Item, index: usize, selected: bool) -> Self {
        // Check if icon is a file path (starts with /)
        let is_icon_path = item.icon.starts_with('/');
        let icon_path = if is_icon_path {
            item.icon.clone()
        } else {
            String::new()
        };

        let icon_initial = if is_icon_path {
            // Use label initial as fallback text
            item.label.chars().next().unwrap_or('#').to_uppercase().next().unwrap_or('#').to_string()
        } else if item.icon.is_empty() {
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
            tags: item
                .tags
                .tags()
                .iter()
                .take(3)
                .map(|t| t.leaf().to_string())
                .collect(),
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
    let class = if item.is_selected {
        "result-item selected"
    } else {
        "result-item"
    };

    let idx = item.index;

    // Shortcut label for first 9 items
    let shortcut = if item.index < 9 {
        format!("\u{2318}{}", item.index + 1)
    } else {
        String::new()
    };

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
                div { class: "result-icon-wrap",
                    img {
                        style: "width: 28px; height: 28px; object-fit: contain; border-radius: 4px;",
                        src: "{item.icon_path}",
                    }
                }
            } else {
                div { class: "result-icon-wrap", "{item.icon}" }
            }

            div { class: "result-body",
                div { class: "result-label",
                    for seg in &item.label_segments {
                        if seg.highlighted {
                            span { class: "match-char", "{seg.text}" }
                        } else {
                            span { "{seg.text}" }
                        }
                    }
                }
                if !item.sub.is_empty() {
                    div { class: "result-sub", "{item.sub}" }
                }
                if has_tags {
                    div { class: "tag-pills",
                        for tag in &item.tags {
                            span { class: "tag-pill", "{tag}" }
                        }
                    }
                }
                if has_note {
                    div {
                        style: "font-size: 11px; color: var(--accent-dim); font-style: italic; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; margin-top: 1px;",
                        "\u{1F4DD} {item.note}"
                    }
                }
            }

            div { class: "result-meta",
                if is_new {
                    span {
                        style: "font-size: 9px; padding: 1px 5px; border-radius: 3px; background: #a6e3a1; color: #1e1e2e; font-weight: 600;",
                        "NEW"
                    }
                }
                if has_rating {
                    span {
                        style: "font-size: 11px; color: #f9e2af;",
                        "{rating_str}"
                    }
                }
                if has_fav {
                    span {
                        style: "color: #f9e2af; font-size: 14px;",
                        "{fav_icon}"
                    }
                }
                span { class: "result-provider-badge", "{item.provider}" }
                if has_extra_actions {
                    span { class: "result-action-count", "{extra_action_label}" }
                }
                if has_shortcut {
                    span { class: "result-shortcut", "{shortcut}" }
                }
            }
        }
    }
}

pub fn highlight_segments(text: &str, positions: &[u32]) -> Vec<TextSegment> {
    if positions.is_empty() {
        return vec![TextSegment {
            text: text.to_string(),
            highlighted: false,
        }];
    }

    let mut segments = Vec::new();
    let mut last = 0;
    let chars: Vec<char> = text.chars().collect();

    for &pos in positions {
        let pos = pos as usize;
        if pos >= chars.len() {
            continue;
        }

        if pos > last {
            segments.push(TextSegment {
                text: chars[last..pos].iter().collect(),
                highlighted: false,
            });
        }

        segments.push(TextSegment {
            text: chars[pos..=pos].iter().collect(),
            highlighted: true,
        });
        last = pos + 1;
    }

    if last < chars.len() {
        segments.push(TextSegment {
            text: chars[last..].iter().collect(),
            highlighted: false,
        });
    }

    segments
}
