use super::structs::Project;
use std::{
    env::{self, set_current_dir},
    path::PathBuf,
    process::Command,
};

pub const FILENAME: &str = "projects.json";
pub type Projects = Vec<Project>;

pub fn add(mut path: Option<PathBuf>, data: &mut Projects) {
    if path.is_none() {
        path = Some(env::current_dir().unwrap());
    }
    let name = path
        .clone()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if !exists(&name, &data) {
        let project = Project {
            name,
            location: path.unwrap(),
        };

        data.push(project);
    }
}

pub fn remove(name: String, data: &mut Projects) {
    let index = data.iter().position(|p| p.name == name);

    if index.is_some() {
        data.remove(index.unwrap());
    }
}

pub fn list(data: &Projects) {
    data.iter().for_each(|p| println!("{:?}", p.name));
}

fn exists(name: &str, data: &Projects) -> bool {
    let project = data.iter().find(|p| p.name == name);

    match project {
        Some(_) => true,
        None => false,
    }
}

pub fn open(name: String, data: &Projects) {
    let project = data.iter().find(|p| p.name == name);

    match project {
        Some(p) => {
            let path = p.location.clone();
            let editor = env::var("EDITOR").unwrap_or("nvim".to_string());
            //goto path
            let res = set_current_dir(path);
            if res.is_err() {
                println!("Error: {}", res.err().unwrap());
                return;
            }
            //open editor
            Command::new(editor).status().unwrap();
        }
        None => println!("Project not found"),
    }
}
