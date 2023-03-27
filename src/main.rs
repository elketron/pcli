use clap::Parser;

mod commands;
mod json_io;
mod projects;
mod xdg;
mod environment;
mod templating;

fn main() {
    let pcli = commands::Cli::parse();

    match pcli.command {
        commands::Command::Project(args) => {
            projects::project_match(args.command.unwrap());
        }
    }
}
