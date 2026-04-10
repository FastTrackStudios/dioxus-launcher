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

#[derive(Clone, PartialEq, Debug)]
pub enum SidebarAction {
    SelectAll,
    Select(String),
    AddInclude(String),
    AddExclude(String),
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
    depth: usize,
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
            depth: node.depth,
            count_label: node.count.map(|c| c.to_string()).unwrap_or_default(),
        })
        .collect();

    rsx! {
        div { class: "w-56 bg-ctp-mantle border-r border-ctp-surface1 flex flex-col overflow-y-auto shrink-0",
            div { class: "px-4 py-3 text-xs font-semibold text-ctp-overlay0 uppercase tracking-wider border-b border-ctp-surface1",
                "Browse"
            }

            div { class: "py-1",
                div {
                    class: if all_active { "flex items-center gap-2 px-4 py-1.5 text-sm text-ctp-blue font-medium cursor-pointer bg-ctp-surface2" } else { "flex items-center gap-2 px-4 py-1.5 text-sm text-ctp-subtext0 cursor-pointer hover:bg-ctp-surface1 hover:text-ctp-text" },
                    onclick: move |_| on_action.call(SidebarAction::SelectAll),
                    span { class: "w-4 text-center text-sm", "\u{2302}" }
                    span { class: "flex-1 truncate", "All" }
                }

                for vn in &visible {
                    {
                        let base = "flex items-center gap-2 py-1.5 text-sm cursor-pointer";
                        let pad = match vn.depth {
                            0 => "pl-4",
                            1 => "pl-9",
                            _ => "pl-14",
                        };
                        let state = if vn.is_active { "text-ctp-blue font-medium bg-ctp-surface2" } else { "text-ctp-subtext0 hover:bg-ctp-surface1 hover:text-ctp-text" };
                        let class = format!("{base} {pad} pr-4 {state}");

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
                                class: "{class}",
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
                                        class: if is_collapsed { "w-4 text-center text-[10px] text-ctp-overlay0 cursor-pointer -rotate-90" } else { "w-4 text-center text-[10px] text-ctp-overlay0 cursor-pointer" },
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
                                    span { class: "w-4 text-center text-sm",
                                        if vn.icon.is_empty() { "\u{2022}" } else { "{vn.icon}" }
                                    }
                                }

                                span { class: "flex-1 truncate", "{vn.label}" }

                                if has_count {
                                    span { class: "text-[11px] text-ctp-overlay0", "{vn.count_label}" }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "px-4 py-2 text-[10px] text-ctp-overlay0 border-t border-ctp-surface1 mt-auto",
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
