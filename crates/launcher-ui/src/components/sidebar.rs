use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SidebarNode {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub depth: usize,
    pub has_children: bool,
    pub count: Option<usize>,
}

/// Sidebar click action with modifier info.
#[derive(Clone, PartialEq, Debug)]
pub enum SidebarAction {
    /// Select "All" (clear filter).
    SelectAll,
    /// Plain click: set as sole include filter.
    Select(String),
    /// Shift+click: add to include filters (OR).
    AddInclude(String),
    /// Alt+click: add to exclude filters (NOT).
    AddExclude(String),
    /// Ctrl+click: remove from filters.
    Remove(String),
}

#[derive(Clone, PartialEq)]
struct VisibleNode {
    id: String,
    label: String,
    icon: String,
    has_children: bool,
    is_active: bool,
    is_collapsed: bool,
    indent_class: String,
    count_label: String,
}

#[component]
pub fn Sidebar(
    nodes: Vec<SidebarNode>,
    active_id: Option<String>,
    on_action: EventHandler<SidebarAction>,
) -> Element {
    let mut collapsed: Signal<Vec<String>> = use_signal(Vec::new);

    let all_active = active_id.is_none();

    let visible: Vec<VisibleNode> = nodes
        .iter()
        .filter(|node| !is_ancestor_collapsed(&nodes, &node.id, &collapsed.read()))
        .map(|node| VisibleNode {
            id: node.id.clone(),
            label: node.label.clone(),
            icon: node.icon.clone(),
            has_children: node.has_children,
            is_active: active_id.as_deref() == Some(&node.id),
            is_collapsed: collapsed.read().contains(&node.id),
            indent_class: match node.depth {
                0 => String::new(),
                1 => "sidebar-indent-1".to_string(),
                _ => "sidebar-indent-2".to_string(),
            },
            count_label: node.count.map(|c| c.to_string()).unwrap_or_default(),
        })
        .collect();

    rsx! {
        div { class: "launcher-sidebar",
            div { class: "sidebar-header", "Browse" }

            div { class: "sidebar-section",
                div {
                    class: if all_active { "sidebar-item active" } else { "sidebar-item" },
                    onclick: move |_| on_action.call(SidebarAction::SelectAll),
                    span { class: "sidebar-item-icon", "\u{2302}" }
                    span { class: "sidebar-item-label", "All" }
                }

                for vn in &visible {
                    {
                        let item_class = format!(
                            "sidebar-item {} {}",
                            vn.indent_class,
                            if vn.is_active { "active" } else { "" }
                        );
                        let id = vn.id.clone();
                        let id_shift = vn.id.clone();
                        let id_alt = vn.id.clone();
                        let id_ctrl = vn.id.clone();
                        let toggle_id = vn.id.clone();
                        let has_children = vn.has_children;
                        let is_collapsed = vn.is_collapsed;
                        let has_count = !vn.count_label.is_empty();

                        rsx! {
                            div {
                                class: "{item_class}",
                                onclick: move |evt: MouseEvent| {
                                    let mods = evt.modifiers();
                                    if mods.ctrl() {
                                        on_action.call(SidebarAction::Remove(id_ctrl.clone()));
                                    } else if mods.alt() {
                                        on_action.call(SidebarAction::AddExclude(id_alt.clone()));
                                    } else if mods.shift() {
                                        on_action.call(SidebarAction::AddInclude(id_shift.clone()));
                                    } else {
                                        on_action.call(SidebarAction::Select(id.clone()));
                                    }
                                },

                                if has_children {
                                    span {
                                        class: if is_collapsed { "sidebar-toggle collapsed" } else { "sidebar-toggle" },
                                        onclick: move |evt| {
                                            evt.stop_propagation();
                                            let mut c = collapsed.write();
                                            if c.contains(&toggle_id) {
                                                c.retain(|x| x != &toggle_id);
                                            } else {
                                                c.push(toggle_id.clone());
                                            }
                                        },
                                        "\u{25BC}"
                                    }
                                } else {
                                    span { class: "sidebar-item-icon",
                                        if vn.icon.is_empty() { "\u{2022}" } else { "{vn.icon}" }
                                    }
                                }

                                span { class: "sidebar-item-label", "{vn.label}" }

                                if has_count {
                                    span { class: "sidebar-item-count", "{vn.count_label}" }
                                }
                            }
                        }
                    }
                }
            }

            // Modifier hints at bottom
            div {
                style: "padding: 8px 16px; font-size: 10px; color: var(--text-muted); border-top: 1px solid var(--border); margin-top: auto;",
                div { "Click: filter" }
                div { "Shift: add" }
                div { "Alt: exclude" }
                div { "Ctrl: remove" }
            }
        }
    }
}

fn is_ancestor_collapsed(nodes: &[SidebarNode], id: &str, collapsed: &[String]) -> bool {
    let Some(node_idx) = nodes.iter().position(|n| n.id == id) else { return false; };
    let node = &nodes[node_idx];
    if node.depth == 0 { return false; }
    for i in (0..node_idx).rev() {
        if nodes[i].depth < node.depth {
            if collapsed.contains(&nodes[i].id) { return true; }
            if nodes[i].depth == 0 { break; }
        }
    }
    false
}
