use crate::xdg;
use crate::cmd;
use crate::structs::data::Data;
use crate::templating::structs::TemplateFiles;

use std::env;

pub const FILE_TEMPLATE_FILE: &str= "/file_templates.json";
pub const FILE_TEMPLATE_XDG_FOLDER: &str = "pcli/file_templates";
type TemData = Data<TemplateFiles>;

pub fn add(path: String, language: Option<String>) {
    let mut data: TemData = Data::read(FILE_TEMPLATE_FILE);
    let dir = xdg::xdg_data_dir(FILE_TEMPLATE_XDG_FOLDER);

    cmd::mv(&path, &dir);

    let name: Vec<&str> = path.split("/").collect();
    let val = name.last().unwrap().to_string();
    let ext: Vec<&str> = val.split(".").collect();

    let file_template = TemplateFiles {
        name: name.last().unwrap_or(&"").to_string(),
        extension: ext.last().unwrap_or(&"").to_string(),
        language: language.unwrap_or("all".to_string())
    };

    data.push(file_template);

    data.write(); 
}

pub fn edit(name: String, language: String) {
    let dir = xdg::xdg_data_dir(FILE_TEMPLATE_XDG_FOLDER);
    let data: TemData = Data::read(FILE_TEMPLATE_FILE);
    
    let template = match data.find(|n| n.name == name && n.language == language) {
        None => std::process::exit(1),
        el => el.unwrap()
    };

    let file = template.name.clone() + "." + &template.extension;
    
    match env::set_current_dir(dir) {
        Ok(_) => cmd::open_editor(Some(file)),
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    };   
}

pub fn list(language: Option<String>, all: bool) {
    let data: TemData = Data::read(FILE_TEMPLATE_FILE);
    
    if all {
        let mut lang: String = "".to_string();
        data.data.iter().for_each(|val| {
            if lang != val.language {
                println!("language: {:?}", val.language);
                lang = val.language.to_owned();
            }
            println!("{:?}", val.name);
        });
    }

    if language.is_some() {
        let lang = language.unwrap();
        data.data.iter().for_each(|val| {
            if val.language == lang {
                println!("{:?}", val.name);
            }
        }); 
    }
}

pub fn use_file(mut name: String, language: String, destination: Option<String>,select: bool) {
    let data: TemData = Data::read(FILE_TEMPLATE_FILE);
    let dir = xdg::xdg_data_dir(FILE_TEMPLATE_XDG_FOLDER);

    if select {
        let mut names: Vec<&String> = vec![];
        data.data.iter().for_each(|val| {
            if val.language == language || val.language == "all".to_string(){
                names.push(&val.name);
            }
        });
        name = cmd::fzf(names);
    }
    
    let template = data.find(|val| 
        val.name == name && 
        val.language == language).unwrap();
    
    let source = dir + "/" + &template.name + "." + &template.extension;

    cmd::cp(&source, &destination.unwrap_or(".".to_string()));
}
