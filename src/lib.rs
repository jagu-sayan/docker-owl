// TODO improve error handling
// #[macro_use]
// extern crate error_chain;
extern crate openssl_probe;
#[macro_use]
extern crate serde_derive;
extern crate shiplift;
extern crate url;
extern crate acme_client;
extern crate termios;
extern crate tera;

pub mod docker;
pub mod plugins;

use std::env;
use docker::DockerEvents;
use plugins::OwlPlugins;

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_version: bool,
    pub flag_endpoint: String,
    pub flag_watch: bool,
    pub flag_only_labels: String,
    pub flag_notify: String,
    pub flag_notify_restart: String,
    pub flag_notify_reload: String,
}

#[derive(Debug)]
pub struct Env {
    pub env_debug: bool
}

impl Default for Env {
    fn default() -> Env {
        let env_debug = env::var("OWL_DEBUG").is_ok();
        Env { env_debug }
    }
}

fn run_plugins(docker: &DockerEvents, plugins: &OwlPlugins) {
    let containers = docker.get_containers_info();
    for container in containers {
        println!("container {:#?}\n", container);

        // 2. Process plugins
        plugins.run_plugins(&container);

        // 3. Notify container
        docker.restart_container(&container.Id);
    }
}

pub fn run(args: &Args, env: &Env) {

    openssl_probe::init_ssl_cert_env_vars();

    let docker = DockerEvents::new(&args.flag_endpoint, &args.flag_only_labels).unwrap();
    let mut plugins = OwlPlugins::default();

    plugins.register_all_plugins(env);

    if !args.flag_watch {
        run_plugins(&docker, &plugins);
        return
    }

    println!("Listening for events");
    for container in docker.watch() {
        println!("Container change -> {:?}\n", container);
        run_plugins(&docker, &plugins);
    }
}
