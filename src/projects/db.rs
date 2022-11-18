use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectsDb {
    pub name: String,
    pub path: PathBuf
}
