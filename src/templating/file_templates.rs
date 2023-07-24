use std::env;
use std::process::Command;
use std::{env::set_current_dir, path::PathBuf};

use crate::cmd::open_editor;
use crate::xdg;

use super::structs::{FileTemplate, Templates};

pub fn add(data: &mut Templates, name: String, language: String) {
    let config_path = xdg::get_data_home(&format!("pcli/file_templates/{}", &language)).unwrap();
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join(&name);

    if !exists(&data, &name, &language) {
        //copy the template to the config path
        Command::new("cp")
            .arg("-r")
            .arg(path)
            .arg(&config_path)
            .spawn()
            .expect("Failed to copy file template");

        let template = FileTemplate {
            name,
            language,
            path: config_path.clone(),
        };

        data.file_templates.push(template);
    }
}

pub fn remove(data: &mut Templates, name: String, language: String) {
    let index = data
        .file_templates
        .iter()
        .position(|p| p.name == name && p.language == language);

    if index.is_some() {
        data.file_templates.remove(index.unwrap());
    }
}

pub fn list(data: &Templates, language: String) {
    let templates = data
        .file_templates
        .iter()
        .filter(|p| p.language == language);

    for template in templates {
        println!("{}", template.name);
    }
}

pub fn use_template(data: &Templates, name: String, language: String, output: PathBuf) {
    let template = data
        .file_templates
        .iter()
        .find(|p| p.name == name && p.language == language);

    if template.is_some() {
        let template = template.unwrap();
        let path = template.path.clone();

        Command::new("cp")
            .arg("-r")
            .arg(path)
            .arg(&output)
            .spawn()
            .expect("Failed to copy file template");
    }
}

pub fn edit(data: &mut Templates, name: String, language: String) {
    let template = data
        .file_templates
        .iter()
        .find(|p| p.name == name && p.language == language);

    if template.is_some() {
        let template = template.unwrap();
        let path = template.path.clone();
        set_current_dir(path.clone()).expect("Failed to change directory");

        open_editor(path, Some(&template.name));
    }
}

pub fn exists(data: &Templates, name: &str, language: &str) -> bool {
    let template = data
        .file_templates
        .iter()
        .find(|p| p.name == name && p.language == language);

    template.is_some()
}
