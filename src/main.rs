use clap::{Parser, Subcommand};

mod json_io;
mod utils;
mod projects;
mod templating;
mod cmd;
mod xdg;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add current folder to DB
    Add { name: Option<String> },
    /// new project
    New { 
        name: String,
        /// template name
        #[arg(long,short)]
        template: Option<String>
    },
    /// template commands
    Template(templating::Templates)
}

fn main() {
    let pcli = CLI::parse();

    match pcli.command {
        Commands::Add {name} => {
            let file_name: String = match name {
                None => utils::get_current_dir(),
                file => file.unwrap()
            };
            
            projects::projects::add(file_name, None)
        },

        Commands::New{ name, template } => {
            projects::projects::new(name, template)
        },
        Commands::Template(tem) => {
            templating::templates_match(tem.command.unwrap())
        }
    }
}
