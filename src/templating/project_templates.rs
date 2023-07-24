use std::{
    env::{self, set_current_dir},
    fmt::format,
    path::PathBuf,
    process::Command,
};

use super::structs::{ProjectTemplate, ProjectType, Templates};

use crate::xdg;

pub fn add(
    data: &mut Templates,
    name: String,
    language: String,
    path: PathBuf,
    project_type: ProjectType,
) {
    let dir = xdg::get_data_home(&format!("pcli/project_templates/{}", &language)).unwrap();

    let config_path = dir.join(&name);

    if !exists(&data, &name, &language) {
        let template = ProjectTemplate {
            name,
            language,
            path: config_path.clone(),
            project_type,
            config: Vec::new(),
        };

        //copy the template to the config path
        Command::new("cp")
            .arg("-r")
            .arg(path)
            .arg(&config_path)
            .spawn()
            .expect("Failed to copy project template");

        data.project_templates.push(template);
    }
}

pub fn remove(data: &mut Templates, name: String, language: String) {
    let index = data
        .project_templates
        .iter()
        .position(|p| p.name == name && p.language == language);

    if index.is_some() {
        data.project_templates.remove(index.unwrap());
    }
}

// list all templates in format "(name).(configuration)"
pub fn list(data: &Templates, language: String) {
    let templates = data
        .project_templates
        .iter()
        .filter(|p| p.language == language);

    for template in templates {
        println!("{}", template.name);
        if !template.config.is_empty() {
            for config in &template.config {
                println!("{}.{}", template.name, config);
            }
        }
    }
}

pub fn add_config(data: &mut Templates, name: String, language: String, config: String) {
    let template = data
        .project_templates
        .iter_mut()
        .find(|p| p.name == name && p.language == language);

    //add config by making a git branch
    let path = template.as_ref().unwrap().path.clone();
    let mut command = Command::new("git");
    command
        .arg("branch")
        .arg(config.clone())
        .current_dir(path);
    command.output().expect("failed to execute process");

    if template.is_some() {
        template.unwrap().config.push(config);
    }
}

pub fn edit(data: &mut Templates, name: String, language: String) {
    // get config from name if there is one
    let config = name.split('.').nth(1);
    let name = name.split('.').nth(0).unwrap();

    let template = data
        .project_templates
        .iter_mut()
        .find(|p| p.name == name && p.language == language);

    //checkout to config

    if template.is_some() {
        let path = template.as_ref().unwrap().path.clone();
        if config.is_some() {
            let mut command = Command::new("git");
            command
                .arg("checkout")
                .arg(config.unwrap().clone())
                .current_dir(path.clone());
            command.output().expect("failed to execute process");
        }

        let editor = env::var("EDITOR").unwrap_or("nvim".to_string());
        //goto path
        let res = set_current_dir(path.clone());
        if res.is_err() {
            println!("Error: {}", res.err().unwrap());
            return;
        }
        //open editor
        Command::new(editor).status().unwrap();

        //git checkout master
        let mut command = Command::new("git");
        command.arg("checkout").arg("master");
        command.current_dir(path);
        command.output().expect("failed to execute process");
    }
}

fn exists(data: &Templates, name: &str, language: &str) -> bool {
    let template = data
        .project_templates
        .iter()
        .find(|p| p.name == name && p.language == language);

    match template {
        Some(_) => true,
        None => false,
    }
}
