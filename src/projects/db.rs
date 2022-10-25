use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct projectsDb {
    pub name: String,
    pub path: PathBuf
}
