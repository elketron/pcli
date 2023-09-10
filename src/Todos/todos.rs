use super::structs::{TodoItem, TodoList, TodoState};

pub fn add_group(data: &mut TodoList, name: String) {
    let id: i32;
    if data.groups.len() == 0 {
        id = 1;
    } else {
        id = data.groups.iter().max_by_key(|g| g.id).unwrap().id + 1;
    }

    data.groups.push(super::structs::TodoGroup {
        id,
        name,
        count: 0,
        count_done: 0,
        count_busy: 0,
    });
}

pub fn add_todo(
    data: &mut TodoList,
    name: String,
    documentation: Option<String>,
    group_id: Option<i32>,
) {
    let id: i32;
    if data.todos.len() == 0 {
        id = 1;
    } else {
        id = data.todos.iter().max_by_key(|t| t.id).unwrap().id + 1;
    }

    data.todos.push(TodoItem {
        id,
        name,
        state: TodoState::NotStarted,
        documentation,
        group_id,
    });

    if let Some(group_id) = group_id {
        let group = data.groups.iter_mut().find(|g| g.id == group_id).unwrap();
        group.count += 1;
    }
}

pub fn list_all(data: &TodoList) {
    for group in data.groups.iter() {
        println!(
            "{} - count {:?}, done {:?}, busy: {:?}",
            group.name, group.count, group.count_done, group.count_busy
        );
        for todo in data.todos.iter().filter(|t| t.group_id == Some(group.id)) {
            println!("  {}", todo.name);
        }
    }
}

pub fn list_groups(data: &TodoList) {
    for group in data.groups.iter() {
        println!(
            "{} - count: {:?}, done: {:?}, busy: {:?}",
            group.name, group.count, group.count_done, group.count_busy
        );
    }
}

pub fn list_todos(data: &TodoList, group_id: i32, todo_state: Option<TodoState>) {
    if let Some(todo_state) = todo_state {
        for todo in data
            .todos
            .iter()
            .filter(|t| t.group_id == Some(group_id) && t.state == todo_state)
        {
            println!("{}", todo.name);
        }
    } else {
        for todo in data.todos.iter().filter(|t| t.group_id == Some(group_id)) {
            println!("{}", todo.name);
        }
    }

    }

pub fn set_state(data: &mut TodoList, id: i32, state: TodoState) {
    let todo = data.todos.iter_mut().find(|t| t.id == id).unwrap();

    data.groups.iter_mut().for_each(|g| {
        if g.id == todo.group_id.unwrap() {
            if todo.state == TodoState::Busy && state == TodoState::Done {
                g.count_busy -= 1;
                g.count_done += 1;
            } else if todo.state == TodoState::Busy && state == TodoState::NotStarted {
                g.count_busy -= 1;
            } else if todo.state == TodoState::NotStarted && state == TodoState::Busy {
                g.count_busy += 1;
            } else if todo.state == TodoState::NotStarted && state == TodoState::Done {
                g.count_done += 1;
            } else if todo.state == TodoState::Done && state == TodoState::NotStarted {
                g.count_done -= 1;
            } else if todo.state == TodoState::Done && state == TodoState::Busy {
                g.count_busy += 1;
                g.count_done -= 1;
            }
        }
    });

    todo.state = state;
}

pub fn remove_todo(data: &mut TodoList, id: i32) {
    data.todos.retain(|t| t.id != id);
}

pub fn remove_group(data: &mut TodoList, id: i32) {
    data.groups.retain(|g| g.id != id);
    data.todos.retain(|t| t.group_id != Some(id));
}
