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

  // println!("args -> {:?}", args);
  if args.flag_version {
    println!("Version 1.0.0");
    return;
  }
  run(&args, &env);
}
