use core::fmt;
use std::{fs, path::PathBuf};

use regex::Regex;

pub struct FileTemplateTransformer {
    template_file: String,
    regex: Regex,
    filename: String,
    language: String,
    project_name: String,
}

impl FileTemplateTransformer {
    pub fn new(
        template_file: &str,
        filename: &str,
        language: &str,
        project_name: &str,
    ) -> FileTemplateTransformer {
        FileTemplateTransformer {
            template_file: template_file.to_string(),
            regex: FileTemplateTransformer::generate_regex(),
            filename: filename.to_string(),
            language: language.to_string(),
            project_name: project_name.to_string(),
        }
    }

    pub fn open(
        template_file: PathBuf,
        language: &str,
        project_name: &str,
    ) -> FileTemplateTransformer {
        let template_file = fs::read_to_string(template_file).unwrap();

        let filename = PathBuf::from(&template_file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        FileTemplateTransformer {
            template_file: template_file.to_string(),
            regex: FileTemplateTransformer::generate_regex(),
            filename: filename,
            language: language.to_string(),
            project_name: project_name.to_string(),
        }
    }

    pub fn transform(&mut self) -> &Self {
        let mut string = String::from(self.template_file.as_str());

        let date = chrono::Local::now().format("%Y-%m-%d").to_string();

        for capture in self.regex.find_iter(self.template_file.as_str()) {
            let capture = capture.as_str();

            let replacement = match capture {
                "{{ filename }}" => self.filename.as_str(),
                "{{ language }}" => self.language.as_str(),
                "{{ projectname }}" => self.project_name.as_str(),
                "{{ date }}" => date.as_str(),
                _ => "",
            };

            if replacement != "" {
                string = string.replace(capture, replacement);
            }
        }

        self.template_file = string;

        self
    }

    pub fn to_file(&self, path: PathBuf) {
        fs::write(path, self.template_file.as_bytes()).unwrap();
    }

    fn generate_regex() -> Regex {
        let template_var_regex = r"\{\{ (language|projectname|filename|date) \}\}";
        let regex = Regex::new(template_var_regex).unwrap();
        regex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let template_file = r#"
        # {{ filename }}
        # {{ projectname }}
        # {{ language }}
        # {{ test }}
        "#;

        let transformer = FileTemplateTransformer::new(template_file, "test", "test", "test");
        let result = transformer.transform();

        let expected = r#"
        # test
        # test
        # test
        # {{ test }}
        "#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_transform_multiple_per_line() {
        let template_file = r#"# {{ filename }} {{ projectname }} {{ language }}"#;

        let transformer = FileTemplateTransformer::new(template_file, "test", "test", "test");
        let result = transformer.transform();

        println!("{}", result);

        let expected = r#"# test test test"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_transform_complex() {
        let templatefile = r#" 
            public class {{ filename }} {
                public static void main(String[] args) {
                    System.out.println("{{ projectname }}");
                    System.out.println("{{ language }}");
                    System.out.println("{{ test }}");

                }
            }
        "#;

        let transformer = FileTemplateTransformer::new(templatefile, "test", "test", "test");
        let result = transformer.transform();

        println!("{}", result);

        let expected = r#" 
            public class test {
                public static void main(String[] args) {
                    System.out.println("test");
                    System.out.println("test");
                    System.out.println("{{ test }}");
                }
            }
        "#;

        assert_eq!(result, expected);
    }
}
