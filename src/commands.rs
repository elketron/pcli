use crate::{projects, templating::commands::TemplateAgs};
use clap::{self, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "pcli")]
#[clap(bin_name = "pcli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Project(projects::commands::ProjectArgs),
    Template(TemplateAgs)
}
