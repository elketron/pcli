use std::env;
use std::process::Command;
use std::{env::set_current_dir, path::PathBuf};

use crate::cmd::open_editor;
use crate::templating::file_template_transfomer;
use crate::xdg;

use super::structs::{FileTemplate, Templates};

pub fn add(data: &mut Templates, name: String, language: String) {
    let config_path = xdg::get_data_home(&format!("file_templates/{}", &language)).unwrap();
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

        let name_without_extension: String = name.split(".").take(1).collect();
        println!("{}", name_without_extension);

        let template = FileTemplate {
            name: name_without_extension.clone(),
            name_with_extension: name,
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

        let path_with_name = path.join(&template.name_with_extension);

        file_template_transfomer::FileTemplateTransformer::open(
            path_with_name,
            language.as_str(),
            name.as_str(),
            output.file_stem().unwrap().to_str().unwrap(),
        )
        .transform()
        .to_file(output);

        // Command::new("cp")
        //     .arg("-r")
        //     .arg(path)
        //     .arg(&output)
        //     .spawn()
        //     .expect("Failed to copy file template");
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
