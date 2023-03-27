use std::path::PathBuf;

#[derive(clap::Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: Option<ProjectCommands>,
}

#[derive(clap::Subcommand)]
pub enum ProjectCommands {
    Add { name: String, path: Option<PathBuf> },
    Remove { name: String },
    List {},
}