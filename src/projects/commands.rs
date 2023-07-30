use std::path::PathBuf;

#[derive(clap::Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: Option<ProjectCommands>,
}

#[derive(clap::Subcommand)]
pub enum ProjectCommands {
    Add {
        path: Option<PathBuf>,
    },
    Remove {
        name: String,
    },
    List {},
    Open {
        name: String,
    },
    Create {
        name: String,
        template: Option<String>,
        language: Option<String>,
    },
    File {
        name: String,
        language: String,
        output: PathBuf,
    },
}
