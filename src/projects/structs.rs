use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub location: PathBuf,
}
