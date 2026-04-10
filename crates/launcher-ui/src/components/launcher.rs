use crate::prelude::*;

use crate::state::LauncherState;
use crate::theme::Theme;
use launcher_core::FilterState;

use super::action_bar::{ActionBar, ActionInfo};
use super::filter_chips::{ChipItem, FilterAction, FilterChips};
use super::notes_editor::NotesEditor;
use super::preset_bar::{PresetBar, PresetChip};
use super::quick_tag::{QuickTag, QuickTagOption};
use super::result_item::{DisplayItem, ResultItem};
use super::search_bar::{LauncherMode, SearchBar};
use super::sidebar::{Sidebar, SidebarAction, SidebarNode};

/// Search mode: either searching items or searching tags/filters.
#[derive(Clone, PartialEq)]
enum SearchMode {
    Items,
    Filters,
}

#[component]
pub fn Launcher(
    state: Signal<LauncherState>,
    theme: Option<Theme>,
    on_close: EventHandler<()>,
) -> Element {
    let theme = theme.unwrap_or_default();
    let stylesheet = use_memo(move || theme.to_stylesheet());

    let mut query = use_signal(|| String::new());
    let mut selected_index = use_signal(|| 0usize);
    let mut mode = use_signal(|| LauncherMode::Full);
    let mut filter = use_signal(FilterState::new);
    let mut search_mode = use_signal(|| SearchMode::Items);
    let mut show_quick_tag = use_signal(|| false);
    let mut show_notes_editor = use_signal(|| false);

    // In filter search mode, query searches tags instead of items
    let results = use_memo(move || {
        let q = query.read().clone();
        let f = filter.read().clone();
        state.read().query_filtered(&q, &f)
    });

    let result_count = results.read().len();

    if selected_index() >= result_count && result_count > 0 {
        selected_index.set(result_count - 1);
    }
    if result_count == 0 {
        selected_index.set(0);
    }

    // Pre-compute display items with all user data
    let display_items: Vec<DisplayItem> = results
        .read()
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let s = state.read();
            let mut di = DisplayItem::from_item(item, idx, idx == selected_index());
            di.is_favorite = s.is_favorite(&item.id);
            di.rating = s.rating(&item.id);
            di.is_new = s.is_recently_added(&item.id);
            let note = s.note(&item.id);
            if !note.is_empty() {
                di.note = note;
            }
            di
        })
        .collect();

    // Action info for selected item — show modifier labels
    let actions: Vec<ActionInfo> = results
        .read()
        .get(selected_index())
        .map(|item| {
            item.actions.iter()
                .map(|a| ActionInfo {
                    name: a.name.clone(),
                    modifier_label: a.modifier.label().to_string(),
                })
                .collect()
        })
        .unwrap_or_default();

    // Tag chips with colors and active state
    let tag_chips: Vec<ChipItem> = {
        let s = state.read();
        let registry = s.engine().tag_registry();
        let f = filter.read();
        registry.root_tags().iter().map(|info| {
            let path = info.tag.path().to_string();
            ChipItem {
                id: path.clone(),
                label: info.display_name.clone(),
                icon: info.icon.clone(),
                is_active: f.include.contains(&path),
                is_excluded: f.exclude.contains(&path),
                color: info.color.clone(),
            }
        }).collect()
    };
    let has_active_filter = !filter.read().is_empty();

    // Preset chips
    let preset_chips: Vec<PresetChip> = state
        .read()
        .presets()
        .iter()
        .map(|p| PresetChip { name: p.name.clone() })
        .collect();
    let mut preset_counter = use_signal(|| 0u32);

    // Filter labels for search bar
    let filter_label_str = {
        let f = filter.read();
        let mut labels = Vec::new();
        for t in &f.include {
            labels.push(format!("+{}", t.rsplit('/').next().unwrap_or(t)));
        }
        for t in &f.exclude {
            labels.push(format!("-{}", t.rsplit('/').next().unwrap_or(t)));
        }
        labels.join(" ")
    };

    // Search mode label
    let mode_label = match *search_mode.read() {
        SearchMode::Items => "Items",
        SearchMode::Filters => "Tags",
    };

    let sidebar_nodes = build_sidebar_nodes(state);

    // --- Event handlers ---

    let on_input = move |value: String| {
        query.set(value);
        selected_index.set(0);
    };

    let on_activate_idx = {
        move |idx: usize| {
            let r = results.read();
            if let Some(item) = r.get(idx) {
                // In filter mode, clicking a result adds it as a tag filter
                if *search_mode.read() == SearchMode::Filters {
                    if let Some(first_tag) = item.tags.tags().first() {
                        filter.write().toggle_include(first_tag.path());
                        drop(r);
                        query.set(String::new());
                        selected_index.set(0);
                        search_mode.set(SearchMode::Items);
                        return;
                    }
                }

                let item = item.clone();
                let q = query.read().clone();
                let should_close = state.read().activate(&item, "activate", &q);
                if should_close {
                    drop(r);
                    on_close.call(());
                }
            }
        }
    };

    let on_key = {
        move |evt: KeyboardEvent| {
            let len = results.read().len();
            let modifiers = evt.modifiers();
            let ctrl = modifiers.ctrl();
            let shift = modifiers.shift();
            let _alt = modifiers.alt();

            match evt.key() {
                Key::ArrowDown => {
                    evt.prevent_default();
                    if len > 0 { selected_index.set((selected_index() + 1) % len); }
                }
                Key::ArrowUp => {
                    evt.prevent_default();
                    if len > 0 { selected_index.set(selected_index().checked_sub(1).unwrap_or(len - 1)); }
                }
                Key::Tab => {
                    evt.prevent_default();
                    // Tab toggles between item search and filter search (Scout-style)
                    let new_mode = match *search_mode.read() {
                        SearchMode::Items => SearchMode::Filters,
                        SearchMode::Filters => SearchMode::Items,
                    };
                    search_mode.set(new_mode);
                    selected_index.set(0);
                }
                Key::Enter => {
                    let r = results.read();
                    if let Some(item) = r.get(selected_index()) {
                        // In filter mode, Enter applies the tag as a filter
                        if *search_mode.read() == SearchMode::Filters {
                            if let Some(first_tag) = item.tags.tags().first() {
                                filter.write().toggle_include(first_tag.path());
                                drop(r);
                                query.set(String::new());
                                selected_index.set(0);
                                search_mode.set(SearchMode::Items);
                                return;
                            }
                        }

                        // Find the action matching the current modifier keys
                        let modifier = launcher_core::ActionModifier::from_modifiers(
                            ctrl, shift, modifiers.alt()
                        );
                        let action_name = item.actions.iter()
                            .find(|a| a.modifier == modifier)
                            .or_else(|| item.actions.first())
                            .map(|a| a.name.clone())
                            .unwrap_or_else(|| "activate".to_string());

                        let keep_open = item.actions.iter()
                            .find(|a| a.modifier == modifier)
                            .map(|a| a.keep_open)
                            .unwrap_or(false);

                        let item = item.clone();
                        let q = query.read().clone();
                        let should_close = state.read().activate(&item, &action_name, &q);
                        if should_close && !keep_open {
                            drop(r);
                            on_close.call(());
                        }
                    }
                }
                Key::Escape => {
                    if !query.read().is_empty() {
                        query.set(String::new());
                        selected_index.set(0);
                    } else if *search_mode.read() == SearchMode::Filters {
                        search_mode.set(SearchMode::Items);
                    } else if !filter.read().is_empty() {
                        filter.write().clear();
                        selected_index.set(0);
                    } else {
                        on_close.call(());
                    }
                }
                Key::Home => { evt.prevent_default(); selected_index.set(0); }
                Key::End => { evt.prevent_default(); if len > 0 { selected_index.set(len - 1); } }
                Key::PageDown => { evt.prevent_default(); if len > 0 { selected_index.set((selected_index() + 10).min(len - 1)); } }
                Key::PageUp => { evt.prevent_default(); selected_index.set(selected_index().saturating_sub(10)); }

                // Ctrl+F: toggle favorite
                Key::Character(ref c) if ctrl && c == "f" => {
                    evt.prevent_default();
                    if let Some(item) = results.read().get(selected_index()) {
                        state.read().toggle_favorite(&item.id);
                    }
                }
                // Ctrl+T: quick tag
                Key::Character(ref c) if ctrl && c == "t" => {
                    evt.prevent_default();
                    if results.read().get(selected_index()).is_some() {
                        show_quick_tag.set(true);
                    }
                }
                // Ctrl+N: notes editor
                Key::Character(ref c) if ctrl && c == "n" => {
                    evt.prevent_default();
                    if results.read().get(selected_index()).is_some() {
                        show_notes_editor.set(true);
                    }
                }
                // Ctrl+E: export all user data
                Key::Character(ref c) if ctrl && c == "e" => {
                    evt.prevent_default();
                    let bundle = state.read().engine().export_all();
                    let export_path = std::path::PathBuf::from(
                        std::env::var("HOME").unwrap_or_else(|_| ".".into())
                    ).join("dioxus-launcher-export.json");
                    match bundle.export_to_file(&export_path) {
                        Ok(()) => tracing::info!(path = %export_path.display(), "Exported user data"),
                        Err(e) => tracing::error!(error = %e, "Export failed"),
                    }
                }
                // Ctrl+L: clear all filters
                Key::Character(ref c) if ctrl && c == "l" => {
                    evt.prevent_default();
                    filter.write().clear();
                    selected_index.set(0);
                }
                // Ctrl+R: random selection
                Key::Character(ref c) if ctrl && c == "r" => {
                    evt.prevent_default();
                    if len > 0 {
                        let random_idx = (std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos() as usize) % len;
                        selected_index.set(random_idx);
                    }
                }
                // Ctrl+H: toggle hide selected item
                Key::Character(ref c) if ctrl && c == "h" => {
                    evt.prevent_default();
                    if let Some(item) = results.read().get(selected_index()) {
                        state.read().toggle_hidden(&item.id);
                    }
                }
                // Ctrl+S: save current filter as preset
                Key::Character(ref c) if ctrl && !shift && c == "s" => {
                    evt.prevent_default();
                    if !filter.read().is_empty() {
                        let f = filter.read().clone();
                        let c = preset_counter() + 1;
                        preset_counter.set(c);
                        state.read().save_preset(&format!("Preset {c}"), &f);
                    }
                }
                // Ctrl+Shift+1-5: set rating on selected item
                Key::Character(ref c) if ctrl && shift && c.len() == 1 => {
                    if let Some(digit) = c.chars().next().and_then(|ch| ch.to_digit(10)) {
                        if digit >= 1 && digit <= 5 {
                            evt.prevent_default();
                            if let Some(item) = results.read().get(selected_index()) {
                                let current = state.read().rating(&item.id);
                                // Toggle: if same rating, clear it
                                let new_rating = if current == digit as u8 { 0 } else { digit as u8 };
                                state.read().set_rating(&item.id, new_rating);
                            }
                        }
                    }
                }
                // Ctrl+1-9 (without shift): activate by position
                Key::Character(ref c) if ctrl && !shift && c.len() == 1 => {
                    if let Some(digit) = c.chars().next().and_then(|ch| ch.to_digit(10)) {
                        if digit >= 1 && digit <= 9 {
                            let idx = (digit - 1) as usize;
                            let r = results.read();
                            if let Some(item) = r.get(idx) {
                                let item = item.clone();
                                let q = query.read().clone();
                                let should_close = state.read().activate(&item, "activate", &q);
                                if should_close { drop(r); on_close.call(()); }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    };

    let on_toggle_mode = move |_: ()| {
        let new = match *mode.read() {
            LauncherMode::Full => LauncherMode::Palette,
            LauncherMode::Palette => LauncherMode::Full,
        };
        mode.set(new);
    };

    let on_clear_tag = move |_: ()| {
        filter.write().clear();
        selected_index.set(0);
    };

    let is_full = matches!(*mode.read(), LauncherMode::Full);
    let launcher_class = if is_full {
        "flex h-screen overflow-hidden bg-ctp-base relative"
    } else {
        "flex h-auto max-h-[460px] overflow-hidden bg-ctp-base relative"
    };

    let query_is_empty = query.read().is_empty() && filter.read().is_empty();
    let show_empty_search = result_count == 0 && !query_is_empty;
    let show_empty_idle = result_count == 0 && query_is_empty;
    let show_recent_header = query_is_empty && result_count > 0;
    let is_filter_mode = *search_mode.read() == SearchMode::Filters;

    // Overlay data for Quick Tag
    let qt_visible = *show_quick_tag.read();
    let (qt_item_id, qt_item_label, qt_current_tags) = if qt_visible {
        let r = results.read();
        if let Some(item) = r.get(selected_index()) {
            let user_tags = state.read().user_tags(&item.id);
            (item.id.clone(), item.label.clone(), user_tags)
        } else {
            show_quick_tag.set(false);
            (String::new(), String::new(), Vec::new())
        }
    } else {
        (String::new(), String::new(), Vec::new())
    };

    let qt_available: Vec<QuickTagOption> = if qt_visible {
        let s = state.read();
        s.engine().tag_registry().all_tags().iter().map(|info| {
            QuickTagOption {
                path: info.tag.path().to_string(),
                label: info.display_name.clone(),
                color: info.color.clone(),
            }
        }).collect()
    } else {
        Vec::new()
    };

    // Overlay data for Notes Editor
    let ne_visible = *show_notes_editor.read();
    let (ne_item_id, ne_item_label, ne_current_note) = if ne_visible {
        let r = results.read();
        if let Some(item) = r.get(selected_index()) {
            let note = state.read().note(&item.id);
            (item.id.clone(), item.label.clone(), note)
        } else {
            show_notes_editor.set(false);
            (String::new(), String::new(), String::new())
        }
    } else {
        (String::new(), String::new(), String::new())
    };

    rsx! {
        // Tailwind CSS is loaded via Stylesheet in the host app (main.rs)
        // The theme.to_stylesheet() is kept as fallback for non-Tailwind hosts

        div { class: "{launcher_class}",
            if is_full {
                Sidebar {
                    nodes: sidebar_nodes,
                    active_id: filter.read().include.first().cloned(),
                    on_action: move |action: SidebarAction| {
                        match action {
                            SidebarAction::SelectAll => {
                                filter.write().clear();
                            }
                            SidebarAction::Select(tag) => {
                                let mut f = filter.write();
                                f.include.clear();
                                f.exclude.clear();
                                f.include.push(tag);
                            }
                            SidebarAction::AddInclude(tag) => {
                                filter.write().toggle_include(&tag);
                            }
                            SidebarAction::AddExclude(tag) => {
                                filter.write().toggle_exclude(&tag);
                            }
                            SidebarAction::Remove(tag) => {
                                filter.write().remove_tag(&tag);
                            }
                        }
                        selected_index.set(0);
                    },
                }
            }

            div { class: "flex-1 flex flex-col min-w-0 overflow-hidden",
                SearchBar {
                    value: query(),
                    on_input: on_input,
                    on_key: on_key,
                    result_count: result_count,
                    mode: mode(),
                    on_toggle_mode: on_toggle_mode,
                    active_tag_label: filter_label_str,
                    on_clear_tag: on_clear_tag,
                }

                // Search mode indicator
                if is_filter_mode {
                    div {
                        style: "padding: 4px 16px; background: var(--accent-dim); font-size: 11px; color: var(--accent); text-transform: uppercase; letter-spacing: 0.05em;",
                        "Tag/Filter Search Mode \u{2014} Tab to switch back \u{2014} Enter to apply"
                    }
                }

                FilterChips {
                    chips: tag_chips,
                    has_active: has_active_filter,
                    on_action: move |action: FilterAction| {
                        match action {
                            FilterAction::Clear => filter.write().clear(),
                            FilterAction::ToggleInclude(tag) => filter.write().toggle_include(&tag),
                            FilterAction::ToggleExclude(tag) => filter.write().toggle_exclude(&tag),
                            FilterAction::Remove(tag) => filter.write().remove_tag(&tag),
                        }
                        selected_index.set(0);
                    },
                }

                PresetBar {
                    presets: preset_chips,
                    show_save: has_active_filter,
                    on_load: move |name: String| {
                        if let Some(fs) = state.read().load_preset(&name) {
                            *filter.write() = fs;
                            selected_index.set(0);
                        }
                    },
                    on_save: move |_: ()| {
                        let f = filter.read().clone();
                        let c = preset_counter() + 1;
                        preset_counter.set(c);
                        let name = format!("Preset {c}");
                        state.read().save_preset(&name, &f);
                    },
                    on_delete: move |name: String| {
                        state.read().delete_preset(&name);
                    },
                }

                div { class: "flex-1 overflow-y-auto px-2 py-1",
                    if show_empty_search {
                        div { class: "flex flex-col items-center justify-center py-12 px-6 text-ctp-overlay0 gap-2",
                            div { class: "text-4xl opacity-40", "\u{1F50D}" }
                            div { class: "text-sm", "No results found" }
                            div { class: "text-xs opacity-60", "Try a different search or clear filters" }
                        }
                    }
                    if show_empty_idle {
                        div { class: "flex flex-col items-center justify-center py-12 px-6 text-ctp-overlay0 gap-2",
                            div { class: "text-4xl opacity-40", "\u{2315}" }
                            div { class: "text-sm", "Start typing to search" }
                            div { class: "text-xs opacity-60",
                                "Tab: search modes \u{00B7} Ctrl+F: fav \u{00B7} Ctrl+Shift+1-5: rate \u{00B7} Ctrl+H: hide \u{00B7} Ctrl+R: random \u{00B7} Ctrl+L: clear"
                            }
                        }
                    }
                    if show_recent_header {
                        div { class: "px-3 pt-2 pb-1 text-[11px] font-semibold text-ctp-overlay0 uppercase tracking-wider", "Recent & Favorites" }
                    }

                    for item in display_items {
                        ResultItem {
                            item: item,
                            on_activate: on_activate_idx,
                        }
                    }
                }

                ActionBar {
                    actions: actions,
                    result_count: result_count,
                    selected_index: selected_index(),
                    mode_label: mode_label.to_string(),
                }
            }

            // Quick Tag overlay
            if qt_visible {
                QuickTag {
                    item_id: qt_item_id,
                    item_label: qt_item_label,
                    current_tags: qt_current_tags,
                    available_tags: qt_available,
                    on_add: move |tag: String| {
                        let r = results.read();
                        if let Some(item) = r.get(selected_index()) {
                            state.read().add_user_tag(&item.id, &tag);
                        }
                    },
                    on_remove: move |tag: String| {
                        let r = results.read();
                        if let Some(item) = r.get(selected_index()) {
                            state.read().remove_user_tag(&item.id, &tag);
                        }
                    },
                    on_close: move |_| show_quick_tag.set(false),
                }
            }

            // Notes Editor overlay
            if ne_visible {
                NotesEditor {
                    item_id: ne_item_id,
                    item_label: ne_item_label,
                    current_note: ne_current_note,
                    on_save: move |note: String| {
                        let r = results.read();
                        if let Some(item) = r.get(selected_index()) {
                            state.read().set_note(&item.id, &note);
                        }
                    },
                    on_close: move |_| show_notes_editor.set(false),
                }
            }
        }
    }
}

fn build_sidebar_nodes(state: Signal<LauncherState>) -> Vec<SidebarNode> {
    let engine = state.read();
    let registry = engine.engine().tag_registry();
    let mut nodes = Vec::new();
    let mut roots: Vec<_> = registry.root_tags().into_iter().collect();
    roots.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    for root in roots {
        add_node_recursive(registry, &root.tag, 0, &mut nodes);
    }
    nodes
}

fn add_node_recursive(
    registry: &launcher_core::TagRegistry,
    tag: &launcher_core::Tag,
    depth: usize,
    nodes: &mut Vec<SidebarNode>,
) {
    let children = registry.children_of(tag);
    let has_children = !children.is_empty();
    let info = registry.info(tag);
    nodes.push(SidebarNode {
        id: tag.path().to_string(),
        label: info.map(|i| i.display_name.clone()).unwrap_or_else(|| tag.leaf().to_string()),
        icon: info.map(|i| i.icon.clone()).unwrap_or_default(),
        depth,
        has_children,
        count: None,
    });
    let mut sorted: Vec<_> = children.into_iter().collect();
    sorted.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    for child in sorted {
        add_node_recursive(registry, &child.tag, depth + 1, nodes);
    }
}
