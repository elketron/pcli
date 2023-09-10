use clap::Parser;

mod Todos;
mod cmd;
mod commands;
mod configuration;
mod environment;
mod json_io;
mod projects;
mod templating;
mod traits;
mod xdg;

fn main() {
    let pcli = commands::Cli::parse();

    match pcli.command {
        commands::Command::Project(args) => {
            projects::project_match(args.command.unwrap());
        }
        commands::Command::Template(args) => {
            templating::template_match(args.command.unwrap(), args.template, args.filetemplate);
        }
        commands::Command::Todo(args) => {
            Todos::todo_match(args.command.unwrap(), args.todo, args.group);
        }
    }
}
