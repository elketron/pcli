use self::commands::ProjectCommands;
use self::projects::{Projects, FILENAME};
use crate::json_io::JsonIO;
use crate::xdg;

pub mod commands;
pub mod projects;
pub mod structs;

pub fn project_match(cmd: ProjectCommands) {
    let mut file = xdg::get_data_home("pcli").unwrap();
    file.set_file_name(FILENAME);
    let json = JsonIO::new(file);

    let mut data: Projects = json.read_json();

    match cmd {
        ProjectCommands::Add { path } => {
            projects::add(path, &mut data);
        }
        ProjectCommands::Remove { name } => {
            projects::remove(name, &mut data);
        }
        ProjectCommands::List {} => {
            projects::list(&data);
        }
        ProjectCommands::Open { name } => {
            projects::open(name, &data);
        }
    }

    json.write_json(&data).expect("failed to write to file");
}
