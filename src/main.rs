use clap::{Parser, Subcommand};

mod json_io;
mod utils;
mod projects;
mod templating;
mod cmd;
mod xdg;
mod structs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// project commands
    Project(projects::Project),
    /// template commands
    Template(templating::Templates)
}

fn main() {
    let pcli = CLI::parse();

    match pcli.command {
        Commands::Project(project) => {
            projects::project_match(project.command.unwrap())
        }, 
        Commands::Template(tem) => {
            templating::templates_match(tem.command.unwrap())
        }
    }
}
