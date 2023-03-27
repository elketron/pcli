// environment variables
//

use std::{
    env::{self, current_dir},
    path::PathBuf,
};

pub fn get_project_dir() -> PathBuf {
    match env::var("PROJECT_DIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => current_dir().unwrap(),
    }
}
