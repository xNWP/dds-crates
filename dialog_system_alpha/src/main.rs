use std::collections::BTreeMap;

use cute_print::cute;
use pause_console::*;
use serde_derive::{Deserialize, Serialize};

fn main() {
    let mut df = DialogFile::empty();
    df.add_node(
        "start",
        DialogNode::CallNode {
            npc_pages: vec![
                Page {
                    text: "hello world, this is the first page of the first node!".to_owned(),
                },
                Page {
                    text: "this is the second page.".to_owned(),
                },
                Page {
                    text: "despite all expectations, this is in fact the third page".to_owned(),
                },
            ],
            next: "response".to_owned(),
        },
    );

    df.add_node(
        "response",
        DialogNode::ResponseNode {
            answers: vec![
                Answer {
                    text: "This is the first option".to_string(),
                    next: "res_1".to_owned(),
                    value: 3,
                },
                Answer {
                    text: "This is the second option".to_string(),
                    next: "res_2".to_owned(),
                    value: 0,
                },
                Answer {
                    text: "This is the third option".to_string(),
                    next: "res_3".to_owned(),
                    value: -1,
                },
            ],
        },
    );

    df.add_node(
        "res_1",
        DialogNode::CallNode {
            npc_pages: vec![Page {
                text: "welcome to the first response".to_owned(),
            }],
            next: "end".to_owned(),
        },
    );
    df.add_node(
        "res_2",
        DialogNode::CallNode {
            npc_pages: vec![Page {
                text: "welcome to the second response".to_owned(),
            }],
            next: "end".to_owned(),
        },
    );
    df.add_node(
        "res_3",
        DialogNode::CallNode {
            npc_pages: vec![Page {
                text: "welcome to the third response".to_owned(),
            }],
            next: "end".to_owned(),
        },
    );

    df.add_node(
        "end",
        DialogNode::EndNode {
            npc_pages: vec![
                Page {
                    text: "welcome to the end node...".to_owned(),
                },
                Page {
                    text: "goodbye!".to_owned(),
                },
            ],
        },
    );

    df.set_start_node("start");
    df.set_npc_name("NpcName");

    //let json = serde_json::to_string_pretty(&df).unwrap();
    //std::fs::write("test_out.json", json).unwrap();

    run_dialog_file(&df);
}

fn print_info(text: impl ToString) {
    let mut cp = cute::CutePrint::new();
    let text = text.to_string();

    cp.add_line("[")
        .add_text("debug")
        .green()
        .add_text("] ")
        .add_text(&text)
        .dim();
    cp.print();
}

fn print_err(text: impl ToString) {
    let mut cp = cute::CutePrint::new();
    let text = text.to_string();
    cp.add_line("[")
        .add_text("error")
        .red()
        .add_text("] ")
        .add_text(&text)
        .dim();
    cp.print();
}

fn run_dialog_file(dialog_file: &DialogFile) {
    print_info("getting start node");
    let active_node = dialog_file.get_node(&dialog_file.start_node);
    if active_node.is_none() {
        print_err(format!(
            "couldn't find start node '{}'",
            dialog_file.start_node
        ));
        return;
    }

    let mut active_node = active_node.unwrap();
    let mut points = 0;

    loop {
        print_info(format!("points: {points}"));

        match active_node {
            DialogNode::CallNode { npc_pages, next } => {
                for page in npc_pages {
                    print_page(dialog_file.npc_name(), page);
                    pause_console!();
                }
                print_info(format!("getting next node: {next}"));
                active_node = dialog_file.get_node(next).expect("couldn't find next node");
            }
            DialogNode::EndNode { npc_pages } => {
                for page in npc_pages {
                    print_page(dialog_file.npc_name(), page);
                    pause_console!();
                }
                print_info("Reached end, exitting.");
                break;
            }
            DialogNode::ResponseNode { answers } => {
                print_info("Entered Response Node");
                let mut idx = 1;
                for a in answers {
                    let mut cp = cute_print::CutePrint::new();
                    let text = cp
                        .add_line(&format!("{idx} ("))
                        .add_text(&format!("{}", a.value));
                    if a.value > 0 {
                        text.green();
                    } else if a.value < 0 {
                        text.red();
                    }
                    text.add_text(" pts): ")
                        .add_text(&format!("{}", a.text))
                        .dim();
                    cp.print();
                    idx += 1;
                }
                loop {
                    print!("selection: ");
                    let selection: usize = text_io::read!();
                    let selection = selection - 1;

                    if selection >= answers.len() {
                        print_err("invalid selection");
                        continue;
                    }

                    let a = &answers[selection];
                    print_info(format!("getting next node: {}", a.next));
                    active_node = dialog_file
                        .get_node(&a.next)
                        .expect("couldn't find next node");

                    points += a.value;
                    break;
                }
            }
        }
    }
}

fn print_page(npc_name: impl ToString, page: &Page) {
    let mut cp = cute_print::CutePrint::new();
    cp.add_line(&npc_name.to_string())
        .add_text(": ")
        .add_text(&page.text)
        .dim();
    cp.print();
}

#[derive(Serialize, Deserialize)]
pub struct DialogFile {
    start_node: String,
    npc_name: String,
    dialog_nodes: BTreeMap<String, DialogNode>,
}
impl DialogFile {
    pub fn add_node(&mut self, name: impl ToString, node: DialogNode) -> Option<DialogNode> {
        self.dialog_nodes.insert(name.to_string(), node)
    }
    pub fn get_node(&self, name: &String) -> Option<&DialogNode> {
        self.dialog_nodes.get(name)
    }

    pub fn start_node(&self) -> &String {
        &self.start_node
    }
    pub fn set_start_node(&mut self, name: impl ToString) {
        self.start_node = name.to_string();
    }

    pub fn npc_name(&self) -> &String {
        &self.npc_name
    }
    pub fn set_npc_name(&mut self, name: impl ToString) {
        self.npc_name = name.to_string();
    }

    pub fn empty() -> Self {
        Self {
            dialog_nodes: BTreeMap::new(),
            start_node: "".to_owned(),
            npc_name: "".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DialogNode {
    CallNode { npc_pages: Vec<Page>, next: String },
    ResponseNode { answers: Vec<Answer> },
    EndNode { npc_pages: Vec<Page> },
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub value: i8,
    pub next: String,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub text: String,
}
