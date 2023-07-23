use crate::xdg;
use serde::{Deserialize, Serialize};
use std::default;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct JsonIO {
    pub path: PathBuf,
}

impl JsonIO {
    pub fn new(path: PathBuf) -> Self {
        JsonIO { path }
    }

    pub fn read_json<T>(&self) -> Vec<T>
    where
        for<'de> T: Deserialize<'de> + Serialize,
    {
        let mut file = match File::open(&self.path) {
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

    pub fn write_json<T>(&self, data: &Vec<T>) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let json_data = serde_json::to_string(data)?;

        let mut file = File::create(&self.path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }
}
