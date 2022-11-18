use serde::{de, Serialize};

use crate::json_io;
use crate::xdg;

pub struct Data<T: de::DeserializeOwned + Serialize> {
    pub filename: String,
    pub data: Vec<T>
}

impl<T: de::DeserializeOwned + Serialize> Default for Data<T> where T: Default {
    fn default() -> Self {
        Data{
            filename: "".to_string(),
            data: vec![]
        }
    }
}

impl<T: de::DeserializeOwned + Serialize> Data<T> {
    pub fn read(filename: &str) -> Self {
        let file = xdg::xdg_data_dir("pcli") + filename;
        let data: Vec<T> = json_io::read_json(&file);
        Data { 
            filename: filename.to_string(),
            data
        }
    }
    
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn write(&self) {
        json_io::write_json(&self.filename, &self.data);
    }

    pub fn exists<F>(&self, f: F) -> bool where F: FnMut(&T) -> bool {
        self.data.iter().any(f)
    }

    pub fn find<F>(&self, f: F) -> Option<&T> where F: FnMut(&&T) -> bool {
        self.data.iter().find(f)
    }

    // not implemented
    fn print_if<F>(&self, f: F) where F: FnMut(&T) {
        self.data.iter().for_each(f);
    }
 }

