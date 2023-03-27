use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum TemplateType {
    FileTemplate,
    ProjectTemplate,
}

#[derive(Serialize, Deserialize)]
struct Template {
    name: String,
    #[serde(rename = "type")]
    template_type: TemplateType,
    path: PathBuf,
    language: String,
}
