use crate::projects;
use clap::{self, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[clap(name = "pcli")]
#[clap(bin_name = "pcli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    List
    Project(projects::commands::ProjectArgs)
}
