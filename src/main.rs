#![feature(await_macro, async_await, futures_api)]
#[macro_use] extern crate log;
extern crate failure;
extern crate docopt;
extern crate docker_owl;

use docopt::Docopt;
use docker_owl::Args;
use docker_owl::Env;
use docker_owl::run;

const USAGE: &str = "
Usage:
  docker-owl [options]
  docker-owl (-h | --help)
  docker-owl --version

Options:
  -h --help                  Show this screen.
  --version                  Show version.
  -e URL, --endpoint URL     Docker api endpoint (tcp|unix://..)
                             [default: unix:///var/run/docker.sock]
  --watch                    Watch for container changes.
  --only-labels=<labels>     Only docker container with specified labels
                             are watched.
                             Each label are separated by whitespace.
                             [default: virtual-host]
  --notify=<command>         Run custom command
  --notify-restart=<id>      Restart container `id`.
  --notify-reload=<id>       Reload container `id`.
";

fn main() {
  let args: Args = Docopt::new(USAGE)
                          .and_then(|d| d.deserialize())
                          .unwrap_or_else(|e| e.exit());
  let env: Env = Env::default();

  trace!("args -> {:?}", args);
  if args.flag_version {
    println!("Version {}", env!("CARGO_PKG_VERSION"));
    return;
  }

    // tokio::run_async(async move {
      run(&args, &env);
    // });

  // if let Err(e) = run(&args, &env) {
  //     for cause in e.causes() {
  //         println!("{}", cause);
  //         if let Some(bt) = cause.backtrace() {
  //           println!("backtrace {}", bt);
  //         }
  //     }
  // }

}
