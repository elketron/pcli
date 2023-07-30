use std::{
    env::{self, set_current_dir},
    path::PathBuf,
    process::Command,
};

pub fn open_editor(path: PathBuf, file: Option<&str>) {
    let editor = env::var("EDITOR").unwrap_or("nvim".to_string());
    //goto path
    let res = set_current_dir(path);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
        return;
    }
    //open editor
    Command::new(editor)
        .arg(file.unwrap_or(""))
        .status()
        .unwrap();
}

pub fn cp(source: PathBuf, destination: PathBuf) {
    let mut command = Command::new("cp");
    command
        .arg("-r")
        .arg(source)
        .arg(destination)
        .output()
        .expect("failed to execute process");
}

pub fn rm(path: PathBuf) {
    let mut command = Command::new("rm");
    command
        .arg("-rf")
        .arg(path)
        .output()
        .expect("failed to execute process");
}

pub fn mkdir(path: PathBuf) {
    let mut command = Command::new("mkdir");
    command
        .arg("-p")
        .arg(path)
        .output()
        .expect("failed to execute process");
}

pub fn git_checkout(path: PathBuf, config: String) {
    let mut command = Command::new("git");
    command.arg("checkout").arg(config).current_dir(path);
    command.output().expect("failed to execute process");
}

pub fn git_init(path: PathBuf) {
    let mut command = Command::new("git");
    command
        .arg("init")
        .current_dir(path)
        .output()
        .expect("failed to execute process");
}

pub fn git_branch(path: PathBuf, config: String) {
    let mut command = Command::new("git");
    command.arg("branch").arg(config).current_dir(path);
    command.output().expect("failed to execute process");
}
