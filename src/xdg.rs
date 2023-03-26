use std::fs;
use std::io;
use std::path::PathBuf;
use xdg::BaseDirectories;

/// return data home or create the folder
pub fn get_data_home(folder: Option<&str>) -> Result<PathBuf, io::Error> {
    let xdg_dirs = BaseDirectories::new().unwrap();

    let data_home = xdg_dirs.get_data_home();
    let dir = format!("pcli/{}", folder.unwrap_or(""));

    let path = data_home.join(dir);

    match fs::create_dir_all(&path) {
        Ok(_) => Ok(path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(path),
            _ => Err(e),
        },
    }
}
