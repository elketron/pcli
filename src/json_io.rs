use crate::xdg;
use serde::{Deserialize, Serialize};
use std::default;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub enum FilePlace {
    #[default]
    XDG,
    Current,
}

#[derive(Default)]
pub struct JsonIO {
    pub folder: String,
    pub place: FilePlace,
}

impl JsonIO {
    pub fn new(folder: &str, place: FilePlace) -> Self {
        JsonIO { folder: folder.to_string(), place }
    }

    pub fn read_json<T>(&self, file_name: &str) -> Vec<T>
    where
        for<'de> T: Deserialize<'de> + Serialize,
    {
        let file = self.get_file_path(file_name);

        let mut file = match File::open(file) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => return Vec::new(),
        };

        match serde_json::from_str(&contents) {
            Ok(v) => v,
            Err(_) => Vec::new(),
        }
    }

    pub fn write_json<T>(
        &self,
        file_name: &str,
        data: &Vec<T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let json_data = serde_json::to_string(data)?;

        let file_path = self.get_file_path(file_name);
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    fn get_file_path(&self, file_name: &str) -> PathBuf {
        let file = match self.place {
            FilePlace::XDG => {
                let path = xdg::get_data_home(&self.folder);
                let file_path = format!("{}/{}", path.unwrap().to_string_lossy(), file_name);
                Path::new(&file_path).to_path_buf()
            }
            FilePlace::Current => {
                let mut path = current_dir().expect("cannot reach current dir");
                path.push(file_name);
                path.as_path().to_path_buf()
            }
        };

        file
    }
}
