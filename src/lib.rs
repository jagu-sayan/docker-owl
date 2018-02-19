#![feature(await_macro, async_await, futures_api)]
extern crate failure;
// extern crate failure_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate tokio;
extern crate futures;
extern crate openssl_probe;
extern crate termios;
extern crate url;
extern crate http;
extern crate shiplift;
extern crate acme_client;
extern crate tera;

pub mod docker;
pub mod plugins;

use std::env;

use failure::Error;

use docker::DockerEvents;
use plugins::OwlPlugins;

use futures::prelude::*;

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


// #[derive(Debug, Fail)]
// pub enum DockerOwlError {
//     #[fail(display = "invalid toolchain name: {}", name)]
//     InvalidToolchainName {
//         name: String,
//     },
//     #[fail(display = "unknown toolchain version: {}", version)]
//     UnknownToolchainVersion {
//         version: String,
//     },
// }

impl Default for Env {
    fn default() -> Env {
        let env_debug = env::var("OWL_DEBUG").is_ok();
        Env { env_debug }
    }
}

async fn run_plugins<'a>(docker: &'a DockerEvents, plugins: &'a OwlPlugins) -> Result<(), Error> {

    let containers = await!(
        docker.get_containers_info()
    );
    for container in containers {
        info!("container {:#?}\n", container);

        // 2. Process plugins
        plugins.run_plugins(&container)?;

        // 3. Notify container
        await!(
            docker.restart_container(&container.id)
        );
    }
    Ok(())
}

async fn run_core(docker: DockerEvents, plugins: OwlPlugins, not_watching: bool) -> std::io::Result<()> {

    if not_watching {
            await!(
                run_plugins(&docker, &plugins)
        ).unwrap();
        return Ok(());
    }

    let mut stream = docker.watch();
    while let Some(_item) = await!(stream.next()) {
        await!(
            run_plugins(&docker, &plugins)
        ).unwrap();
    }

    Ok(())
}

pub fn run(args: &Args, env: &Env) {

    openssl_probe::init_ssl_cert_env_vars();

    let docker = DockerEvents::new(&args.flag_endpoint, &args.flag_only_labels).unwrap();

    let mut plugins = OwlPlugins::default();
    plugins.register_all_plugins(env);

    let not_watching = !args.flag_watch;
    info!("Listening for events");

    tokio::run(
        run_core(docker, plugins, not_watching)
        .map_err(|e| eprintln!("Oh no: {}", e))
        .boxed()
        .compat()
    );
}
