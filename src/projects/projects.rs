use super::structs::Project;
use crate::{cmd::git_checkout, templating};
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

pub fn create(data: &mut Projects, name: String, template: String, language: String) {
    let project_dir = env::var("PROJECT_DIR").unwrap_or("".to_string());
    let template_data = templating::get_templates();

    let template_name = template.split('.').nth(0).unwrap();
    let template_config = template.split('.').nth(1);

    let template = template_data
        .project_templates
        .iter()
        .find(|p| p.name == template_name && p.language == language);

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

        if template_config.is_some() {
            git_checkout(
                template.unwrap().path.clone(),
                template_config.unwrap().to_string(),
            );
        } else {
            git_checkout(template.unwrap().path.clone(), "master".to_string());
        }

        //mv template to project_dir
        let mut command = Command::new("cp");
        command
            .arg("-r")
            .arg(template.unwrap().path.clone())
            .arg(path.join(name.clone()));
        command.output().expect("failed to execute process");

        let mut command = Command::new("rm");
        command.arg("-rf").arg(path.join(name.clone()).join(".git"));

        let mut command = Command::new("git");
        command
            .arg("init")
            .current_dir(path.join(name.clone()))
            .output()
            .expect("failed to execute process");

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
