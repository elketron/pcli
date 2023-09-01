use std::path::PathBuf;
use walkdir::WalkDir;

use regex::Regex;

pub struct ProjectTemplateTransformer {
    project_path: PathBuf,
}

impl ProjectTemplateTransformer {
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }

    pub fn transform(&self) {
        let regex = self.generate_regex();

        // walk project_path
        // if file or directory = projectname = substitutionString -> replace with project_name

        for entry in WalkDir::new(&self.project_path) {
            let entry = entry.unwrap();

            let path = entry.path();
            let path_str = path.to_str().unwrap();

            if regex.is_match(path_str) {
                let new_path = regex.replace_all(path_str, "project_name");
                std::fs::rename(path, new_path.to_string()).unwrap();
            }
        }
    }

    fn generate_regex(&self) -> Regex {
        let substitution_string: &str = r"$\{project_name\}";

        let regex = Regex::new(substitution_string).unwrap();

        regex
    }
}
