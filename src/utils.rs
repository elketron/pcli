// file with utility functions

use std::env;

pub fn get_current_dir() -> String {
    let current_path = env::current_dir().unwrap();
    current_path.file_name().unwrap().to_string_lossy().to_string()
}
