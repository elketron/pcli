use std::{env::current_dir, fs::create_dir_all, path::PathBuf};

use crate::json_io::JsonIO;

use self::{commands::TodoCommand, structs::TodoList};

pub mod commands;
mod structs;
mod todos;

pub fn todo_match(cmd: TodoCommand, todo: bool, group: bool) {
    let file_name = PathBuf::from("./.proj/todos.json").canonicalize();

    let canonical_path = match file_name {
        Ok(path) => path,
        Err(_) => {
            create_dir_all("./.proj").unwrap();
            current_dir().unwrap().join("./.proj/todos.json")
        }
    };

    let json = JsonIO::new(canonical_path);

    let mut todo_list: TodoList = json.read_json().unwrap_or_default();

    match cmd {
        TodoCommand::Add {
            name,
            documentation,
            group_id,
        } => {
            if group {
                todos::add_group(&mut todo_list, name);
            } else {
                todos::add_todo(&mut todo_list, name, documentation, group_id);
            }
        }
        TodoCommand::List {
            all,
            group_id,
            todo_state,
        } => {
            if all {
                todos::list_all(&todo_list);
            } else if todo {
                todos::list_todos(&todo_list, group_id.unwrap(), todo_state);
            } else if group {
                todos::list_groups(&todo_list);
            }
        }
        TodoCommand::SetState { id, state } => {
            if todo {
            todos::set_state(&mut todo_list, id, state);
            } else {
                println!("You can only set the state of a todo");
            }
        }
        TodoCommand::Remove { id } => {
            if todo {
                todos::remove_todo(&mut todo_list, id)
            } else if group {
                todos::remove_group(&mut todo_list, id)
            }
        }
    }

    json.write_json(&todo_list).unwrap();
}
