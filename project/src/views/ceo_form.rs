use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Head {
    pub img_src: &'static str,
    pub name: &'static str,
    pub title: &'static str,
    pub department: &'static str,
}

const AVAILABLE_HEADS: &[Head] = &[
    Head {
        img_src: "alice.png",
        name: "Alice",
        title: "VP of Engineering",
        department: "Engineering",
    },
    Head {
        img_src: "bob.png",
        name: "Bob",
        title: "Head of Sales",
        department: "Sales",
    },
    Head {
        img_src: "carol.png",
        name: "Carol",
        title: "CFO",
        department: "Finance",
    },
];

pub fn CeoForm() -> Element {
    let selected_heads = use_signal(|| Vec::<String>::new());
    let focused_head = use_signal(|| None::<String>);
    let selected_questions = use_signal(|| HashMap::<String, String>::new());

    let questions_map: HashMap<&str, Vec<&str>> = [
        ("Engineering", vec!["Ship dates?", "Tech debt?", "Hiring needs?"]),
        ("Sales", vec!["Pipeline?", "Top accounts?", "Churn?"]),
        ("Finance", vec!["Cash runway?", "Forecast?", "Burn rate?"]),
    ]
    .into_iter()
    .collect();

    let get_head = |name: &str| AVAILABLE_HEADS.iter().find(|h| h.name == name);

    rsx! {
        div { class: "p-4 space-y-6",
            h2 { "Available Heads" }
            div { class: "flex flex-wrap gap-4",
                for head in AVAILABLE_HEADS.iter() {
                    if !selected_heads.read().contains(&head.name.to_string()) {
                        let head_name = head.name.to_string();
                        let selected_heads_sig = selected_heads.clone();

                        div { class: "border rounded-lg p-2 w-40 text-center bg-gray-50 hover:shadow",
                            img { src: "{head.img_src}", class: "mx-auto rounded-full mb-2 w-16 h-16" }
                            p { "{head.name}" }
                            p { "{head.title}" }
                            p { "Dept: {head.department}" }
                            button {
                                class: "bg-green-500 text-white px-2 py-1 rounded mt-2",
                                onclick: move |_| {
                                    selected_heads_sig.write().push(head_name.clone());
                                },
                                "Add"
                            }
                        }
                    }
                }
            }

            h2 { "Selected Heads" }
            div { class: "flex flex-wrap gap-4",
                for name in selected_heads.read().iter() {
                    {
                        let current_name = name.clone();
                        let selected_heads_sig = selected_heads.clone();
                        let focused_head_sig = focused_head.clone();
                        let selected_questions_sig = selected_questions.clone();
                        let is_focused = focused_head.read().as_deref() == Some(current_name.as_str());

                        if let Some(head) = get_head(&current_name) {
                            div {
                                class: format!(
                                    "border rounded-lg p-2 w-40 text-center {} {}",
                                    "bg-gray-50 hover:shadow",
                                    if is_focused {
                                        "border-blue-500 bg-blue-100"
                                    } else {
                                        ""
                                    }
                                ),

                                img { src: "{head.img_src}", class: "mx-auto rounded-full mb-2 w-16 h-16" }
                                p { "{head.name}" }
                                p { "{head.title}" }
                                p { "Dept: {head.department}" }

                                button {
                                    class: "bg-red-500 text-white px-2 py-1 rounded mt-2 mr-1",
                                    onclick: {
                                        let current_name_remove = current_name.clone();
                                        move |_| {
                                            selected_heads_sig.write().retain(|n| n != &current_name_remove);
                                            focused_head_sig.write().take();
                                            selected_questions_sig.write().remove(&current_name_remove);
                                        }
                                    },
                                    "Remove"
                                }

                                button {
                                    class: "bg-blue-500 text-white px-2 py-1 rounded mt-2",
                                    onclick: {
                                        let current_name_focus = current_name.clone();
                                        move |_| focused_head_sig.set(Some(current_name_focus.clone()))
                                    },
                                    if is_focused {
                                        "Focused"
                                    } else {
                                        "Focus"
                                    }
                                }

                                if is_focused {
                                    if let Some(questions) = questions_map.get(head.department) {
                                        let selected_questions_sig = selected_questions.clone();
                                        let current_name_question = current_name.clone();
                                        
                                        select {
                                            class: "border rounded p-1 mt-2 w-full",
                                            onchange: move |evt| {
                                                selected_questions_sig.write()
                                                    .insert(current_name_question.clone(), evt.value().clone());
                                            },
                                            option { value: "", "Select a question" }
                                            for q in questions {
                                                option {
                                                    value: "{q}",
                                                    selected: selected_questions.read()
                                                        .get(&current_name)
                                                        .map(|v| v == q)
                                                        .unwrap_or(false),
                                                    "{q}"
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

            button {
                class: "bg-gray-800 text-white px-4 py-2 rounded mt-4",
                onclick: move |_| {
                    println!("Selected heads and questions:");
                    for name in selected_heads.read().iter() {
                        if let Some(h) = get_head(name) {
                            let question = selected_questions.read()
                                .get(name)
                                .cloned()
                                .unwrap_or_default();
                            println!("{} - {} - {} | Question: {}",
                                h.name, h.title, h.department, question);
                        }
                    }
                },
                "Submit"
            }
        }
    }
}