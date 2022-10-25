// does folder templates and file templates
pub mod projects;

#[derive(clap::Args)]
pub struct Templates {
    #[clap(subcommand)]
    pub command: Option<TemplateCommands>
}

#[derive(clap::Subcommand)]
pub enum TemplateCommands {
    /// add a template
    Add {
        #[arg(short='t', long="type")]
        template_type: TemplateTypes,
        #[arg(short='p', long="project_type")]
        project_type: Option<projects::ProjectTypes> 
    },
    /// edit a template
    ///template name
    Edit {
        name: String,
        #[arg(short='t', long="type")]
        template_type: TemplateTypes,
    },
    ///lists all templates (default: --all)
    List {
        language: Option<String>,
        #[arg(short='t', long="type")]
        template_type: TemplateTypes,
        project_type: Option<projects::ProjectTypes>,
        all: Option<bool>
    }
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum TemplateTypes {
    Project,
    File
}

pub fn templates_match (cmd: TemplateCommands) {
    match cmd {
        TemplateCommands::Add {template_type, project_type} => {
            match template_type {
                TemplateTypes::File => {},
                TemplateTypes::Project => projects::add(project_type) 
            }
        },
        TemplateCommands::Edit {template_type, name} => {
            match template_type {
                TemplateTypes::File => {},
                TemplateTypes::Project => {}
            }
        },
        TemplateCommands::List {language,template_type,project_type ,all} => {
            match template_type {
                TemplateTypes::File => {},
                TemplateTypes::Project => {}
            }
        }
    }
}
