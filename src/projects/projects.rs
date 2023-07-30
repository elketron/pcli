use super::structs::Project;
use crate::{
    cmd::{self, git_checkout, git_init},
    templating::{
        self,
        structs::{self, ProjectTemplate},
    },
};
use std::{
    env::{self, set_current_dir},
    path::PathBuf,
    process::Command,
};

pub const FILENAME: &str = "projects.json";
pub type Projects = Vec<Project>;

pub fn add(path: Option<PathBuf>, data: &mut Projects) {
    let path = match path {
        Some(p) => {
            if "." == p.to_str().unwrap() {
                env::current_dir().unwrap()
            } else {
                p
            }
        }
        None => env::current_dir().unwrap(),
    };

    let name = path
        .clone()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if !exists(&name, &data) {
        let project = Project {
            name,
            location: path,
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
    data.iter().for_each(|p| println!("{}", p.name));
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

fn process_template(template: Option<&str>, language: Option<&str>) -> Option<ProjectTemplate> {
    if let Some(template_str) = template {
        let template_data = templating::get_templates();
        let mut template_config = "master";

        let mut template_parts = template_str.split('.');
        let template_name = template_parts.next().unwrap();

        if let Some(config) = template_parts.next() {
            template_config = config;
        }

        if let Some(template) = template_data
            .project_templates
            .iter()
            .find(|p| p.name == template_name && p.language == language.unwrap_or_default())
        {
            git_checkout(template.path.clone(), template_config.to_string());
            return Some(template.clone());
        } else {
            println!("Template not found.");
        }
    }

    None
}

pub fn create(
    data: &mut Projects,
    name: String,
    template: Option<String>,
    language: Option<String>,
) {
    let project_dir = env::var("PROJECT_DIR").unwrap_or("".to_string());
    println!("project_dir: {}", project_dir);

    if !exists(&name, &data) {
        let path = if project_dir.is_empty() {
            env::current_dir().unwrap()
        } else {
            PathBuf::from(project_dir)
        };
        let project = Project {
            name: name.clone(),
            location: path.clone(),
        };

        let template = process_template(template.as_deref(), language.as_deref());

        //mv template to project_dir
        if template.is_some() {
            cmd::cp(template.unwrap().path.clone(), path.join(name.clone()));
            cmd::rm(path.join(name.clone()).join(".git"));
        } else {
            cmd::mkdir(path.join(name.clone()));
        }

        git_init(path.join(name.clone()));
        data.push(project);
    }
}

pub fn file(_data: &Projects, name: String, language: String, output: PathBuf) {
    let template_data = templating::get_templates();

    let template = template_data
        .file_templates
        .iter()
        .find(|p| p.name == name && p.language == language);

    if template.is_some() {
        let path = template.as_ref().unwrap().path.clone();
        let mut command = Command::new("cp");
        command
            .arg("-r")
            .arg(path.clone())
            .arg(output.join(name.clone()));
        command.output().expect("failed to execute process");
    }
}
