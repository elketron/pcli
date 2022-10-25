use crate::json_io;
use crate::xdg;
use crate::cmd;

use serde::{Serialize, Deserialize};
use std::env;

pub const PROJECTS_TEMPLATE_FILE: &str= "/project_templates.json";
pub const TEMPLATE_XDG_FOLDER: &str = "pcli/project_templates";

// the type of project 
// type can't be used as it is a reserved key word
#[derive(Serialize, Deserialize, Debug, clap::ValueEnum, Clone, Copy)]
pub enum ProjectTypes {
    Cli,
    Desktop,
    Server,
    Library,
    Webapp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectTemplate {
    pub name: String,
    pub template_type: ProjectTypes,
    pub commands: Vec<String>,
}

pub fn add(project_type: Option<ProjectTypes>) {
    let file = xdg::xdg_data_dir("pcli") + PROJECTS_TEMPLATE_FILE;
    let mut data: Vec<ProjectTemplate> = json_io::read_json(&file);
    // get data
    let current_dir = env::current_dir().unwrap();
    let folder_name = &current_dir.into_iter().last().unwrap().to_str().unwrap().to_string();

    // move to data dir
    let dir = xdg::xdg_data_dir(TEMPLATE_XDG_FOLDER);
    cmd::mv(current_dir.to_str().unwrap(), &dir);

    let template = ProjectTemplate {
        name: folder_name.to_string(),
        template_type: project_type.unwrap_or(ProjectTypes::Cli),
        commands: vec![]
    };

    data.push(template);

    json_io::write_json(&file, data);
}
