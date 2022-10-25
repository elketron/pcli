// file with utility functions

use std::env;
use serde::{Serialize, Deserialize, de};
use crate::json_io;
use serde_json;

pub fn get_current_dir() -> String {
    let current_path = env::current_dir().unwrap();
    current_path.file_name().unwrap().to_string_lossy().to_string()
}

