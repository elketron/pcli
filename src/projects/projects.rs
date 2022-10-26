use crate::projects::db;
use crate::templating::projects::{
    PROJECTS_TEMPLATE_FILE, 
    TEMPLATE_XDG_FOLDER,    
    ProjectTemplate
};
use crate::xdg;
use crate::json_io;
use crate::cmd;

use std::fs;
use std::env;
use std::path::Path;

const PROJECTS_FILE: &str = "/projects.json";

pub fn add(name: String, path: Option<String>) {
    let filename = xdg::xdg_data_dir("pcli") + PROJECTS_FILE;

    let project = db::projectsDb {
        name,
        path: match path {
            None => std::env::current_dir().unwrap(),
            p => Path::new(&p.unwrap()).to_path_buf()
        } 
    };

    let mut data: Vec<db::projectsDb> = json_io::read_json(&filename);
    
    // if project already exists
    if data.iter().any(|i| i.path == project.path) {
        std::process::exit(1);
    }

    data.push(project);

    json_io::write_json(&filename, data);
}

pub fn new(name: String, template: Option<String>) {
    let template_file = xdg::xdg_data_dir("pcli") + PROJECTS_TEMPLATE_FILE;
    let template_data: Vec<ProjectTemplate> = json_io::read_json(&template_file);
    let project = xdg::get_projects_dir() + &name;
    
    match template {
        None => {
            default_project(project.clone());
        },
        tem => {        
            if template_data.iter().any(|i| &i.name == tem.as_ref().unwrap()) {
               let source = xdg::xdg_data_dir(TEMPLATE_XDG_FOLDER) + &tem.unwrap();
                cmd::mv(&source, &project); 
            } else {
                std::process::exit(1);
            }
        }
    };
    
    add(name, Some(project));
}

pub fn open(mut name: String, select: bool) {
    let filename = xdg::xdg_data_dir("pcli") + PROJECTS_FILE;
    let data: Vec<db::projectsDb> = json_io::read_json(&filename);
    let project: &db::projectsDb;

    if select {
        let mut names: Vec<&String> = vec![];
        for project in data.iter() {
            names.push(&project.name);
        }
        name = cmd::fzf(names);
    }
    
    project = match data.iter().find(|i| i.name == name) {
        None => std::process::exit(1),
        el => el.unwrap()
    };
    
    let path = Path::new(&project.path);
    
    match env::set_current_dir(path) {
        Ok(_) => cmd::open_editor(),
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    };   
}

fn default_project(path: String) {
    fs::create_dir_all(&path)
        .expect("could not create directory");
    fs::create_dir(path + "/.proj")
        .expect("could not create .proj directory");
}

