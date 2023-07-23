use crate::projects;
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
}
