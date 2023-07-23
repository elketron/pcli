use super::structs::Project;
use std::{env, path::PathBuf};

pub const FILENAME: &str = "projects.json";
pub type Projects = Vec<Project>;

pub fn add(name: String, mut path: Option<PathBuf>, data: &mut Projects) {
    if path.is_none() {
        path = Some(env::current_dir().unwrap());
    }

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
