use std::vec::Vec;
use shiplift;
use ::Env;
use self::encrypt::Acme;
use self::template::Template;

pub mod encrypt;
pub mod template;

pub type DockerContainerDetails = shiplift::rep::ContainerDetails;

pub trait OwlPlugin {
    fn get_name(&self) -> &'static str;
    fn should_process(&self, &DockerContainerDetails) -> bool;
    fn process(&self, details: &DockerContainerDetails);
}

pub struct OwlPlugins {
    pub plugins: Vec<Box<OwlPlugin>>,
}

impl OwlPlugins {

    pub fn register_plugin(&mut self, plugin: Box<OwlPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn register_all_plugins(&mut self, env: &Env) {
        self.register_plugin(Acme::new(env.env_debug));
        self.register_plugin(Template::new());
    }

    pub fn run_plugins(&self, details: &DockerContainerDetails) {
        for plugin in &self.plugins {
            if plugin.should_process(details) {
                println!("Process plugin {}", plugin.get_name());
                plugin.process(details);
            }
        }
    }
}

impl Default for OwlPlugins {
    fn default() -> OwlPlugins {
        OwlPlugins { plugins: Vec::new() }
    }
}