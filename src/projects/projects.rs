use crate::projects::db;
use crate::templating::projects::{
    PROJECTS_TEMPLATE_FILE, 
    TEMPLATE_XDG_FOLDER,    
};
use crate::xdg;
use crate::cmd;
use crate::structs::data::Data;

use std::fs;
use std::env;
use std::path::Path;

const PROJECTS_FILE: &str = "/projects.json";
type ProjData = Data<db::ProjectsDb>;

pub fn add(name: String, path: Option<String>) {
    let project = db::ProjectsDb {
        name,
        path: match path {
            None => std::env::current_dir().unwrap(),
            p => Path::new(&p.unwrap()).to_path_buf()
        } 
    };

    let mut data: ProjData= Data::read(PROJECTS_FILE);
    
    if data.exists(|i| i.path == project.path){
        std::process::exit(1);
    }

    data.push(project);
    
    data.write();
}

pub fn new(name: String, template: Option<String>) {
    let data: ProjData = Data::read(PROJECTS_TEMPLATE_FILE);
    let project_dir = xdg::get_projects_dir() + &name;
    
    match template {
        None => {
            default_project(project_dir.clone());
        },
        name => {        
            if data.exists(|i| &i.name == name.as_ref().unwrap()) {
                let source = xdg::xdg_data_dir(TEMPLATE_XDG_FOLDER) + &name.unwrap();
                cmd::cp(&source, &project_dir); 
            } else {
                std::process::exit(1);
            }
        }
    };
    
    add(name, Some(project_dir));
}

pub fn open(mut name: String, select: bool) {
    let data: ProjData = Data::read(PROJECTS_FILE);
    let project: &db::ProjectsDb;

    if select {
        let mut names: Vec<&String> = vec![];
        for project in data.data.iter() {
            names.push(&project.name);
        }
        name = cmd::fzf(names);
    }
    
    project = match data.find(|i| i.name == name) {
        None => std::process::exit(1),
        el => el.unwrap()
    };
    
    let path = Path::new(&project.path);
    
    match env::set_current_dir(path) {
        Ok(_) => cmd::open_editor(None),
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

