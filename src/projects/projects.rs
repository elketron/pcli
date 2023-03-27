use super::structs::Project;
use chrono::Utc;
use std::{env, path::PathBuf};

pub const FILENAME: &str = "projects.json";

pub struct Projects {
    pub data: Vec<Project>,
}

impl Default for Projects {
    fn default() -> Self {
        Projects { data: vec![] }
    }
}

impl Projects {
    pub fn new(data: Vec<Project>) -> Self {
        Projects { data }
    }

    pub fn add(&mut self, name: String, mut path: Option<PathBuf>) {
        if path.is_none() {
            path = Some(env::current_dir().unwrap());
        }

        if !self.exists(&name) {
            let project = Project {
                name,
                created_at: Utc::now(),
                modified_at: Utc::now(),
                location: path.unwrap(),
            };

            self.data.push(project);
        }
    }

    pub fn remove(&mut self, name: String) {
        let index = self.data.iter().position(|p| p.name == name);

        if index.is_some() {
            self.data.remove(index.unwrap());
        }
    }

    pub fn list(&self) {
        self.data.iter().for_each(|p| println!("{:?}", p.name));
    }

    fn exists(&self, name: &str) -> bool {
        let project = self.data.iter().find(|p| p.name == name);

        match project {
            Some(_) => true,
            None => false,
        }
    }
}
