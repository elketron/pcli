use std::path::PathBuf;

use super::structs::ProjectType;

#[derive(clap::Args)]
pub struct TemplateAgs {
    #[clap(subcommand)]
    pub command: Option<SubCommand>,

    #[clap(short, long)]
    pub template: bool,

    #[clap(short, long)]
    pub filetemplate: bool,
}

#[derive(clap::Subcommand)]
pub enum SubCommand {
    #[clap(name = "add")]
    /// add a template.
    /// for file templates you only need to use the name and language fields.
    /// for project templates you need to use all fields
    Add {
        name: String,
        language: String,
        path: Option<PathBuf>,
        project_type: Option<ProjectType>,
    },
    #[clap(name = "remove")]
    Remove { name: String, language: String },
    #[clap(name = "list")]
    List { language: String },
    /// use only for file templates
    #[clap(name = "use")]
    Use {
        name: String,
        language: String,
        output: PathBuf,
    },

    /// add config for project templates
    #[clap(name = "config")]
    Config {
        name: String,
        language: String,
        config: String,
    },

    #[clap(name = "edit")]
    Edit { name: String, language: String },
}
