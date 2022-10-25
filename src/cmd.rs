use std::process::Command;


pub fn mv(source: &str, dest: &str){
    Command::new("mv")
        .args([source, dest])
        .spawn()
        .expect("could not move folder/ file");
}
