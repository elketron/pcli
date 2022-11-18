// does folder templates and file templates
pub mod projects;
pub mod files;
mod structs;

#[derive(clap::Args)]
pub struct Templates {
    #[clap(subcommand)]
    pub command: Option<TemplateCommands>
}

#[derive(clap::Subcommand)]
pub enum TemplateCommands {
    /// add a template
    Add {
        #[arg(short='P',long)]
        path: Option<String>,
        #[arg(short='t', long="type")]
        template_type: TemplateTypes,
        #[arg(short='p', long="project_type")]
        project_type: Option<structs::ProjectTypes>,
        #[arg(short,long)]
        /// default is "all"
        language: Option<String>
    },
    /// edit a template
    ///template name
    Edit {
        #[arg(short,long)]
        name: String,
        #[arg(short='t', long="type")]
        template_type: TemplateTypes,
        #[arg(short,long)]
        language: String
    },
    ///lists all templates (default: --all)
    List {
        #[arg(short,long)]
        language: Option<String>,
        #[arg(short='t', long="type")]
        template_type: TemplateTypes,
        #[arg(short,long)]
        project_type: Option<structs::ProjectTypes>,
        #[arg(short,long)]
        all: bool
    },
    Use {
        #[arg(short,long)]
        name: Option<String>,
        #[arg(short,long)]
        language: Option<String>,
        #[arg(short,long)]
        destination: Option<String>,
        #[arg(short,long)]
        select: bool
    }
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum TemplateTypes {
    Project,
    File
}

pub fn templates_match (cmd: TemplateCommands) {
    match cmd {
        TemplateCommands::Add {template_type, project_type, language, path} => {
            match template_type {
                TemplateTypes::File => {files::add(path.unwrap(), language)},
                TemplateTypes::Project => projects::add(project_type, language) 
            }
        },
        TemplateCommands::Edit {template_type, name, language} => {
            match template_type {
                TemplateTypes::File => { files::edit(name, language)},
                TemplateTypes::Project => { projects::edit(name, language)}
            }
        },
        TemplateCommands::List {language,template_type,project_type ,all} => {
            match template_type {
                TemplateTypes::File => {files::list(language, all)},
                TemplateTypes::Project => {projects::list(language, project_type, all)}
            }
        },
        TemplateCommands::Use {name, language, destination, select} => {
            files::use_file(
                name.unwrap_or("".to_string()), 
                language.unwrap_or("all".to_string()),
                destination,
                select
            )
        }
    }
}
