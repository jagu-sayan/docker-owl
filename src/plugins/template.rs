use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::boxed::Box;

use failure::{Error, SyncFailure, ResultExt};

use tera::Tera;

use super::DockerContainerDetails;
use super::OwlPlugin;

#[derive(Debug)]
pub struct Template {}

impl Template {

    pub fn new() -> Box<OwlPlugin + Send + Sync> {
        Box::new(Template {})
    }

    pub fn add_template_files(tera: &mut Tera, src: &str, files_name: &str) -> Result<(), Error> {

        let template_files: Vec<(PathBuf, Option<&str>)> = files_name.split_whitespace()
            .map(|path| (Path::new(src).join(path), Some(path)))
            .collect();
        tera.add_template_files(template_files).map_err(SyncFailure::new)?;

        Ok(())
    }

    pub fn render_template_files(tera: &Tera, dest: &str, details: &DockerContainerDetails) -> Result<(), Error> {

        for template_name in tera.templates.keys() {
            info!("tera template name -> {:?}", template_name);

            let result = tera.render(template_name, &details).map_err(SyncFailure::new)?;
            let path = Path::new(dest).join(template_name);

            let mut buffer = File::create(&path).context(format!("File {:?}", &path))?;
            buffer.write_all(result.as_bytes())?;
        }

        Ok(())
    }

}

impl OwlPlugin for Template {

    fn get_name(&self) -> &'static str {
        const NAME: &str = "Template";
        NAME
    }

    fn should_process(&self, details: &DockerContainerDetails) -> bool {
        details.config.labels.as_ref().map_or(false, |labels|
            labels.contains_key("owl-template-enable")
            && labels.contains_key("owl-template-files")
            && labels.contains_key("owl-template-src")
            && labels.contains_key("owl-template-dest")
        )
    }

    fn process(&self, details: &DockerContainerDetails) -> Result<(), Error> {
        let label = &details.config.labels.as_ref().expect("You need to call should_process before call this function");

        let files = label.get("owl-template-files").unwrap();
        let src = label.get("owl-template-src").unwrap();
        let dest = label.get("owl-template-dest").unwrap();


        let tera = &mut Tera::default();
        Template::add_template_files(tera, src, files)?;
        Template::render_template_files(tera, dest, details)?;

        Ok(())
    }
}
