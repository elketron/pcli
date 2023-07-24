use clap::Parser;

mod commands;
mod environment;
mod json_io;
mod projects;
mod templating;
mod traits;
mod xdg;
mod cmd;

fn main() {
    let pcli = commands::Cli::parse();

    match pcli.command {
        commands::Command::Project(args) => {
            projects::project_match(args.command.unwrap());
        }
        commands::Command::Template(args) => {
            templating::template_match(args.command.unwrap(), args.template, args.filetemplate);
        }
    }
}
