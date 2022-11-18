// command that have to do with projects and project templates

pub mod db;
pub mod projects;

use crate::utils;

#[derive(clap::Args)]
pub struct Project {
    #[clap(subcommand)]
    pub command: Option<ProjectCommands>
}

#[derive(clap::Subcommand)]
pub enum ProjectCommands {
    /// add current folder to DB
    Add { name: Option<String> },
    /// new project
    New { 
        name: String,
        /// template name
        #[arg(long,short)]
        template: Option<String>
    },
    Open {
        name: Option<String>,
        #[arg(long,short)]
        select: bool
    },
}

pub fn project_match(cmd: ProjectCommands) {
    match cmd {
        ProjectCommands::Add {name} => {
            let file_name: String = match name {
                None => utils::get_current_dir(),
                file => file.unwrap()
            };
            
            projects::add(file_name, None)
        },
        ProjectCommands::New{ name, template } => {
            projects::new(name, template)
        },
        ProjectCommands::Open {name, select} => {
            if name == None && !select {
                println!("at least one of the two must have a value");
                std::process::exit(1);
            }
            projects::open(name.unwrap_or("".to_string()), select)
        }
    }
}


