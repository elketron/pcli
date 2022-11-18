use std::process::{Command, Stdio};
use std::io::Write;
use std::env;

pub fn mv(source: &str, dest: &str) {
    Command::new("mv")
        .args([source, dest])
        .spawn()
        .expect("could not move folder/ file");
}

pub fn cp(source: &str, dest: &str) {
    Command::new("cp")
        .args([source, dest])
        .spawn()
        .expect("failed to copy folder/file");
}

pub fn fzf<T: std::fmt::Display>(vals: Vec<T>) -> String {
    let mut child = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdin = child.stdin.as_mut().unwrap();

    for element in vals {
        let val = format!("{}{}", element, "\n");
        child_stdin.write_all(&val.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().unwrap();

    let mut val = String::new();
    val.push_str(std::str::from_utf8(&output.stdout).unwrap());
    val.pop();
    return val;
}

pub fn open_editor(file: Option<String>) -> std::process::ExitStatus {
    // get editor 
    let editor = match env::var("EDITOR") {
        Ok(val) => val,
        Err(_) => "".to_string()
    };
    let arg = editor + " " + &file.unwrap_or("".to_string());
    let command = Command::new("/usr/bin/sh")
        .args(["-c", &arg])
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");

    command
}
