// read and write functions for json
//

use serde::{Serialize,de};
use std::fs::File;
use std::io::Write;
use std::fs;

pub fn write_json<T: Serialize>(filename: &str, data: T) {
    println!("{filename}");
    let json = serde_json::to_string(&data).unwrap();
    
    let mut json_file = File::create(filename).unwrap();

    write!(&mut json_file, "{}", json.as_str()).unwrap();

}

pub fn read_json<T: de::DeserializeOwned>(filename: &str) -> Vec<T> {
    let json = match fs::read_to_string(filename) {
        Ok(value) => Some(value.to_string()),
        Err(_) => None
    };

    match json {
        None => vec![],
        j => serde_json::from_str(&j.unwrap()).unwrap()
    }
}
