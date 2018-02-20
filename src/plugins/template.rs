use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;

use tera::Tera;

use super::DockerContainerDetails;
use super::OwlPlugin;

#[derive(Debug)]
pub struct Template {}

impl Template {

    pub fn new() -> Box<Template> {
        Box::new(Template {})
    }

    pub fn add_template_files(tera: &mut Tera, src: &str, files_name: &str) {

        let template_files: Vec<(PathBuf, Option<&str>)> = files_name.split_whitespace()
            .map(|path| (Path::new(src).join(path), Some(path)))
            .collect();

        println!("ADD files");
        tera.add_template_files(template_files).unwrap();
    }

    pub fn render_template_files(tera: &Tera, dest: &str, details: &DockerContainerDetails) {

        for template_name in tera.templates.keys() {
            println!("tera !!! {:?}", tera);
            println!("tera -> {:?}", template_name);

            let result = tera.render(template_name, &details).unwrap();
            let path = Path::new(dest).join(template_name);

            let mut buffer = File::create(path).unwrap();
            buffer.write_all(result.as_bytes()).unwrap();
        }
    }

}

impl OwlPlugin for Template {

    fn get_name(&self) -> &'static str {
        const NAME: &str = "Template";
        NAME 
    }

    fn should_process(&self, details: &DockerContainerDetails) -> bool {
        details.Config.Labels.contains_key("owl-template-enable")
        && details.Config.Labels.contains_key("owl-template-files")
        && details.Config.Labels.contains_key("owl-template-src")
        && details.Config.Labels.contains_key("owl-template-dest")
    }

    fn process(&self, details: &DockerContainerDetails) {
        let label = &details.Config.Labels;

        let files = label.get("owl-template-files").unwrap();
        let src = label.get("owl-template-src").unwrap();
        let dest = label.get("owl-template-dest").unwrap();


        let tera = &mut Tera::default();
        Template::add_template_files(tera, src, files);
        Template::render_template_files(tera, dest, details);
    }
}