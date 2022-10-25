// functions for loading the folder and files structure

use std::env;
use std::path;
use std::fs;

const XDG_DATA_HOME: &str = "/.local/share/";
const HOME: &str = "HOME";

fn get_home_dir() -> String {
    match env::var(HOME) {
        Ok(val) => val,
        Err(_) => "".to_string()
    }
}

pub fn get_projects_dir() -> String {
    match env::var("PROJECTS_DIR") {
        Ok(val) => val,
        Err(_) => "".to_string()
    }
} 

// return the data dir and create it if it does not exist
pub fn xdg_data_dir(name: &str) -> String {
    let home_dir = get_home_dir();

    let data_dir: String = match env::var("XDG_DATA_HOME") {
        Ok(val) => val + "/" + name,
        Err(_) => home_dir + XDG_DATA_HOME + name
    };

    if !path::Path::new(&data_dir).is_dir() {
        fs::create_dir_all(&data_dir)
            .expect("cannot create directory");
    }

    data_dir
}
