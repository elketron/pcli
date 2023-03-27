use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProjectType {
    Cli,
    API,
    Desktop,
    Server,
    Library,
    Webapp,
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub project_templates: Vec<ProjectTemplate>,
    pub file_templates: Vec<FileTemplate>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub language: String,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct FileTemplate {
    pub name: String,
    pub ext: String,
    pub language: String,
    pub path: PathBuf,
}
