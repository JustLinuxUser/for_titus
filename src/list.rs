use std::ops::Add;

use crossterm::event::{KeyCode, KeyEvent};
use ego_tree::{tree, NodeId};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListState},
    Frame,
};

struct ListNode {
    name: &'static str,
    command: &'static str,
}

pub struct CustomList {
    inner_tree: ego_tree::Tree<ListNode>,
    visit_stack: Vec<NodeId>,
    list_state: ListState,
}

impl CustomList {
    pub fn new() -> Self {
        let tree = tree!(ListNode {
            name: "root",
            command: ""
        } => {
            ListNode {
                name: "Eza",
                command: "eza -la"
            },
            ListNode {
                name: "sleep and Eza",
                command: "sleep 2 && eza -la"
            },
            ListNode {
                name: "Just ls, nothing special, trust me",
                command: include_str!("commands/special_ls.sh"),
            },
            ListNode {
                name: "Test Category",
                command: ""
            } => {
                ListNode {
                    name: "sleep, eza, sleep, eza",
                    command: "sleep 1 && eza -la && sleep 1 && eza -la && echo Bonus eza comming... && sleep 1 && eza -la"
                },
                ListNode {
                    name: "Just open neovim :), because I can",
                    command: "nvim"
                },
                ListNode {
                    name: "Recursion?",
                    command: "cargo run"
                },
            },
        });
        let root_id = tree.root().id();
        Self {
            inner_tree: tree,
            visit_stack: vec![root_id],
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let curr = self
            .inner_tree
            .get(*self.visit_stack.last().unwrap())
            .unwrap();
        let mut items = vec![];

        if !self.at_root() {
            items.push(Line::from("  ..").blue());
        }
        for node in curr.children() {
            if node.has_children() {
                items.push(Line::from(format!("  {}", node.value().name)).blue());
            } else {
                items.push(Line::from(format!("  {}", node.value().name)).red());
            }
        }

        let list = List::new(items)
            .highlight_symbol(">>> ")
            .highlight_style(Style::default().reversed())
            .block(Block::default().borders(Borders::ALL).title("List"));

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> Option<&'static str> {
        match event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.handle_key_down();
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.handle_key_up();
                None
            }
            KeyCode::Enter => self.handle_enter(),
            _ => None,
        }
    }
    fn handle_key_up(&mut self) {
        self.list_state
            .select(Some(self.list_state.selected().unwrap().saturating_sub(1)));
    }
    fn handle_key_down(&mut self) {
        let curr = self
            .inner_tree
            .get(*self.visit_stack.last().unwrap())
            .unwrap();

        let count = if self.at_root() {
            // the .. does not exist, when there is
            // nowhere to go up
            curr.children().count() - 1
        } else {
            curr.children().count()
        };
        self.list_state
            .select(Some(self.list_state.selected().unwrap().add(1).min(count)));
    }

    fn at_root(&self) -> bool {
        self.visit_stack.len() == 1
    }

    fn handle_enter(&mut self) -> Option<&'static str> {
        let curr = self
            .inner_tree
            .get(*self.visit_stack.last().unwrap())
            .unwrap();
        let mut selected = self.list_state.selected().unwrap();
        if !self.at_root() && selected == 0 {
            self.visit_stack.pop();
            self.list_state.select(Some(0));
            return None;
        }

        if !self.at_root() {
            selected -= 1; // we need to account for ..
        }
        for (idx, node) in curr.children().enumerate() {
            if idx == selected {
                if node.has_children() {
                    self.visit_stack.push(node.id());
                    self.list_state.select(Some(0));
                    return None;
                } else {
                    return Some(node.value().command);
                }
            }
        }
        None
    }
}
