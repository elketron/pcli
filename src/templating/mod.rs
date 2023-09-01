use crate::json_io::JsonIO;
use crate::xdg;

use self::{
    commands::{SubCommand, TemplateAgs},
    structs::Templates,
};

pub mod commands;
mod file_templates;
mod project_templates;
pub mod file_template_transfomer;
pub mod structs;
pub mod project_template_transformer;

const FILENAME: &str = "templates.json";

pub fn get_templates() -> Templates {
    let mut file = xdg::get_data_home("pcli").unwrap();
    file.set_file_name(FILENAME);
    let json = JsonIO::new(file);

    json.read_json().unwrap_or_default()
}

pub fn template_match(cmd: SubCommand, template: bool, filetemplate: bool) {
    if !template && !filetemplate {
        println!("Please specify either --template or --filetemplate");
        return;
    }

    let mut file = xdg::get_data_home("pcli").unwrap();
    file.set_file_name(FILENAME);
    let json = JsonIO::new(file);

    let mut data: Templates = json.read_json().unwrap_or_default();

    match cmd {
        commands::SubCommand::Add {
            name,
            language,
            path,
            project_type,
        } => {
            if template {
                project_templates::add(
                    &mut data,
                    name,
                    language,
                    path.unwrap(),
                    project_type.unwrap(),
                );
            } else if filetemplate {
                file_templates::add(&mut data, name, language);
            }
        }
        commands::SubCommand::Remove { name, language } => {
            if template {
                project_templates::remove(&mut data, name, language);
            } else if filetemplate {
                file_templates::remove(&mut data, name, language);
            }
        }
        commands::SubCommand::List { language } => {
            if template {
                project_templates::list(&data, language);
            } else if filetemplate {
                file_templates::list(&data, language);
            }
        }
        commands::SubCommand::Use {
            name,
            language,
            output,
        } => {
            if filetemplate {
                file_templates::use_template(&data, name, language, output);
            } else {
                println!("Please only use for file templates");
            }
        }
        commands::SubCommand::Config {
            name,
            language,
            config,
        } => {
            if template {
                project_templates::add_config(&mut data, name, language, config);
            } else {
                println!("Please only config for project templates");
            }
        }
        commands::SubCommand::Edit { name, language } => {
            if template {
                project_templates::edit(&mut data, name, language);
            } else if filetemplate {
                file_templates::edit(&mut data, name, language);
            }
        }
    }

    json.write_json(&data).expect("failed to write to file");
}
