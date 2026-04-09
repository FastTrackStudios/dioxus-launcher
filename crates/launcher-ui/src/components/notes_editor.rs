use crate::prelude::*;

/// Inline notes editor overlay for the selected item.
#[component]
pub fn NotesEditor(
    item_id: String,
    item_label: String,
    current_note: String,
    on_save: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    let mut note_text = use_signal(|| current_note.clone());

    rsx! {
        div {
            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 100;",
            onclick: move |_| on_close.call(()),

            div {
                style: "background: var(--bg-primary); border: 1px solid var(--border); border-radius: var(--radius-lg); padding: 16px; width: 420px; display: flex; flex-direction: column; gap: 10px; box-shadow: var(--shadow-lg);",
                onclick: move |evt| evt.stop_propagation(),

                div {
                    style: "font-size: 14px; font-weight: 600; color: var(--text-primary);",
                    "Notes: {item_label}"
                }

                textarea {
                    style: "background: var(--bg-input); border: 1px solid var(--border); border-radius: var(--radius-md); padding: 8px 10px; color: var(--text-primary); font-size: 13px; font-family: var(--font-family); outline: none; width: 100%; min-height: 120px; resize: vertical; line-height: 1.5;",
                    autofocus: true,
                    value: "{note_text}",
                    oninput: move |evt| note_text.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            on_close.call(());
                        }
                        // Ctrl+Enter to save
                        if evt.key() == Key::Enter && evt.modifiers().ctrl() {
                            on_save.call(note_text.read().clone());
                            on_close.call(());
                        }
                    },
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",

                    span {
                        style: "font-size: 11px; color: var(--text-muted);",
                        "Ctrl+Enter to save \u{00B7} Esc to cancel"
                    }

                    div {
                        style: "display: flex; gap: 6px;",

                        button {
                            style: "padding: 4px 12px; border-radius: var(--radius-sm); background: var(--bg-secondary); color: var(--text-secondary); border: 1px solid var(--border); cursor: pointer; font-size: 12px; font-family: var(--font-family);",
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }

                        button {
                            style: "padding: 4px 12px; border-radius: var(--radius-sm); background: var(--accent); color: var(--text-inverse); border: none; cursor: pointer; font-size: 12px; font-family: var(--font-family);",
                            onclick: move |_| {
                                on_save.call(note_text.read().clone());
                                on_close.call(());
                            },
                            "Save"
                        }
                    }
                }
            }
        }
    }
}
