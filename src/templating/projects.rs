use crate::xdg;
use crate::cmd;
use crate::structs::data::Data;
use crate::templating::structs::{ProjectTemplate, ProjectTypes};

use std::env;

pub const PROJECTS_TEMPLATE_FILE: &str= "/project_templates.json";
pub const TEMPLATE_XDG_FOLDER: &str = "pcli/project_templates";
type TemProjData = Data<ProjectTemplate>;

// the type of project 
// type can't be used as it is a reserved key word
pub fn add(project_type: Option<ProjectTypes>, language: Option<String>) {
    let lang = language.unwrap_or("all".to_string());
    let mut data: TemProjData = Data::read(PROJECTS_TEMPLATE_FILE);
    // get data
    let current_dir = env::current_dir().unwrap();
    let folder_name = &current_dir.into_iter().last().unwrap().to_str().unwrap().to_string();

    if data.exists(|tem| tem.name == folder_name.to_owned() && tem.language == lang) {
        std::process::exit(1);
    }

    // move to data dir
    let dir = generate_dir_name(&folder_name, &lang);
    cmd::mv(current_dir.to_str().unwrap(), &dir);

    let template = ProjectTemplate {
        name: folder_name.to_string(),
        language: lang,
        template_type: project_type.unwrap_or(ProjectTypes::Cli),
        commands: vec![]
    };

    data.push(template);

    data.write();
}

pub fn edit(name: String, language: String) {
    let data: TemProjData = Data::read(PROJECTS_TEMPLATE_FILE);
    
    let template = match data.find(|n| n.name == name && n.language == language) {
        None => std::process::exit(1),
        el => el.unwrap()
    };

    let folder = generate_dir_name(&template.name, &template.language);
    
    match env::set_current_dir(folder) {
        Ok(_) => cmd::open_editor(None),
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    };   
}

pub fn list(language: Option<String>, project_type: Option<ProjectTypes>, all: bool) {  
    let data: TemProjData = Data::read(PROJECTS_TEMPLATE_FILE);

    if all {
        data.data.iter().for_each(|template| println!("{:?}", template));
        std::process::exit(0);
    }

    if language.is_some() && project_type.is_some() {
        let lang = language.unwrap();
        let p_type = project_type.unwrap();

        data.data.iter().for_each(|template| {
            print_template(template, || lang == template.language && p_type == template.template_type);
        });
        std::process::exit(0);
    }

    if language.is_some() {
        let lang = language.unwrap();
        
        data.data.iter().for_each(|template| {
            print_template(template, || lang == template.language);
        });
        std::process::exit(0);
    }

    if project_type.is_some() {
        let p_type = project_type.unwrap();

        data.data.iter().for_each(|template| print_template(template, || p_type == template.template_type));
        std::process::exit(0);
    }
}

fn print_template<F>(template: &ProjectTemplate, f: F) where F: Fn() -> bool {
    if f() {
        println!("{:?}", template);
    }
}

fn generate_dir_name(name: &String, language: &str) -> String {
    xdg::xdg_data_dir(TEMPLATE_XDG_FOLDER) + "/" + name + "." + language
}
