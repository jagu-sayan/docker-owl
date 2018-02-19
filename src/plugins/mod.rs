use std::vec::Vec;
use std::boxed::Box;
use failure::{Error};
use shiplift;
use crate::Env;
use self::encrypt::Acme;
use self::template::Template;

pub mod encrypt;
pub mod template;

pub type DockerContainerDetails = shiplift::rep::ContainerDetails;

pub trait OwlPlugin {
    fn get_name(&self) -> &'static str;
    fn should_process(&self, details: &DockerContainerDetails) -> bool;
    fn process(&self, details: &DockerContainerDetails) -> Result<(), Error>;
}

pub struct OwlPlugins {
    pub plugins: Vec<Box<OwlPlugin + Send + Sync>>,
}


// #[derive(Debug)]
// struct OwlPluginError {
//     inner: Context<OwlPluginErrorKind>,
// }

// #[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
// enum OwlPluginErrorKind {
//     // A plain enum with no data in any of its variants
//     //
//     // For example:
//     // ...
//     #[fail(display = "Encrypt plugin fail.")]
//     LetsencryptError,
// }

// impl OwlPluginError {
//     pub fn kind(&self) -> OwlPluginErrorKind {
//         *self.inner.get_context()
//     }
// }

// impl From<OwlPluginErrorKind> for OwlPluginError {
//     fn from(kind: OwlPluginErrorKind) -> OwlPluginError {
//         OwlPluginError { inner: Context::new(kind) }
//     }
// }

// impl From<Context<OwlPluginErrorKind>> for OwlPluginError {
//     fn from(inner: Context<OwlPluginErrorKind>) -> OwlPluginError {
//         OwlPluginError { inner: inner }
//     }
// }

// impl Fail for OwlPluginError {
//     fn cause(&self) -> Option<&Fail> {
//         self.inner.cause()
//     }

//     fn backtrace(&self) -> Option<&Backtrace> {
//         self.inner.backtrace()
//     }
// }

// impl Display for OwlPluginError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         Display::fmt(&self.inner, f)
//     }
// }


impl OwlPlugins {

    pub fn register_plugin(&mut self, plugin: Box<OwlPlugin + Send + Sync>) {
        self.plugins.push(plugin);
    }

    pub fn register_all_plugins(&mut self, env: &Env) {
        self.register_plugin(Acme::new(env.env_debug));
        self.register_plugin(Template::new());
    }

    pub fn run_plugins(&self, details: &DockerContainerDetails) -> Result<(), Error> {
        for plugin in &self.plugins {
            if plugin.should_process(details) {
                info!("Process plugin {}", plugin.get_name());
                plugin.process(details)?;
            }
        }
        Ok(())
    }

}

impl Default for OwlPlugins {
    fn default() -> OwlPlugins {
        OwlPlugins { plugins: Vec::new() }
    }
}
