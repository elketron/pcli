use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, clap::ValueEnum, Clone, Copy, PartialEq)]
pub enum ProjectTypes {
    Cli,
    Desktop,
    Server,
    Library,
    Webapp,
}

impl Default for ProjectTypes {
    fn default() -> Self { ProjectTypes::Cli }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectTemplate {
    pub name: String,
    pub language: String,
    pub template_type: ProjectTypes,
    pub commands: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TemplateFiles {
    pub name: String,
    pub extension: String,
    pub language: String
}


