use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, ValueEnum, Debug, PartialEq, Eq)]
pub enum TodoState {
    NotStarted,
    Busy,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub name: String,
    pub state: TodoState,
    pub documentation: Option<String>,
    pub group_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoGroup {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub count_done: i32,
    pub count_busy: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
    pub groups: Vec<TodoGroup>,
}

impl Default for TodoList {
    fn default() -> Self {
        Self {
            todos: Vec::new(),
            groups: Vec::new(),
        }
    }
}
