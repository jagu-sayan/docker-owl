# Docker owl

## Usage
```
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

Arguments:
  template - path to a template to generate
  dest - path to a write the template. If not specfied, STDOUT is used

Environment Variables:
  DOCKER_HOST - default value for -endpoint
  DOCKER_CERT_PATH - directory path containing key.pem, cert.pm and ca.pem
  DOCKER_TLS_VERIFY - enable client TLS verification

Let's encrypt labels:
  - letsencrypt-host     Specify the host used by letsencrypt
                         [default: localhost]
  - letsencrypt-mail     Mail used in letsencrypt certificates.
                         [default: demo@localhost]
  - letsencrypt-keysize  Determines the size of the requested key.
                         [default: 4096]

Template label:
  - 
  - Any label prefixed
  By convention all letsencrypt label use `ssl-label` as prefix.
  - letsencrypt-host     Specify the host used by letsencrypt
                         [default: localhost]
  - letsencrypt-mail     Mail used in letsencrypt certificates.
                         [default: demo@localhost]
  - letsencrypt-keysize  Determines the size of the requested key.
                         [default: 4096]
```
