use self::structs::Project;
use self::{commands::ProjectCommands, projects::Projects};
use crate::json_io::{JsonIO};

pub mod commands;
pub mod projects;
pub mod structs;

pub fn project_match(cmd: ProjectCommands) {
    let json = JsonIO::default();

    let data: Vec<Project> = json.read_json(projects::FILENAME);
    let mut projects = Projects::new(data);

    match cmd {
        ProjectCommands::Add { name, path } => {
            projects.add(name, path);
        }
        ProjectCommands::Remove { name } => {
            projects.remove(name);
        }
        ProjectCommands::List {} => {
            projects.list();
        }
    }

    json.write_json(projects::FILENAME, &projects.data)
        .expect("failed to write to file");
}
