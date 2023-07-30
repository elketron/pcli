use std::path::PathBuf;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, ValueEnum, Debug)]
pub enum ProjectType {
    Cli,
    API,
    Desktop,
    Server,
    Library,
    Webapp,
    Mobile,
    Game,
    Embedded,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Templates {
    pub project_templates: Vec<ProjectTemplate>,
    pub file_templates: Vec<FileTemplate>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectTemplate {
    pub name: String,
    pub language: String,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub path: PathBuf,
    pub config: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FileTemplate {
    pub name: String,
    pub language: String,
    pub path: PathBuf,
}
