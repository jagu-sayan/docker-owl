<p align="center"><a href="https://github.com/jagu-sayan/docker-owl">
  <img alt="Docker owl" width="64" src="media/docker-owl-logo.svg" />
</a></p>

Docker owl
==========

> Watch docker container events

[![CircleCI](https://circleci.com/gh/jagu-sayan/docker-owl.svg?style=svg)](https://circleci.com/gh/jagu-sayan/docker-owl)

## Usage
```
Usage:
  docker-owl [options]
  docker-owl (-h | --help)
  docker-owl --version

Options:
  -h --help                  Show this screen
  --version                  Show version
  -e URL, --endpoint URL     Docker api endpoint (tcp|unix://..)
                             [default: unix:///var/run/docker.sock]
  --watch                    Watch for container changes
  --only-labels=<labels>     Only docker container with specified labels
                             are watched
                             Each label are separated by whitespace
                             [default: virtual-host]
  --notify=<command>         Run custom command
  --notify-restart=<id>      Restart container `id`
  --notify-reload=<id>       Reload container `id`

Environment Variables:
  OWL_DEBUG - enable debug mode
  DOCKER_HOST - default value for -endpoint
  DOCKER_CERT_PATH - directory path containing key.pem, cert.pm and ca.pem
  DOCKER_TLS_VERIFY - enable client TLS verification

Docker let's encrypt labels:
  - letsencrypt-enable       Use `encrypt` plugin
  - letsencrypt-dest         Path where the certification files are written
                             [default: ./certs]
  - letsencrypt-webroot      Destination path of http challenge
                             e.g. ${letsencrypt-webroot}/.well-known/acme-challenge/
                             [default: ./www]
  - letsencrypt-challenge    A verification challenge (values: http, dns)
                             [default: dns]
  - letsencrypt-hostname     Specify the host used by letsencrypt
                             [default: localhost]
  - letsencrypt-mail         Mail used in letsencrypt certificates
                             [default: None]
  - letsencrypt-keysize      Determines the size of the requested key
                             [default: 4096]
  - letsencrypt-dh-group     Use pre-defined DHE groups (values: ffdhe2048, ffdhe3072, ffdhe4096)
                             [default: None]

Docker let's encrypt environment variables:
LETSENCRYPT_FORCE=true       Force renewal of letsencrypt certificate

Docker template labels:
  - owl-template-enable      Use `template` plugin
  - owl-template-files       List of template filenames (separated by whitespaces)
  - owl-template-src         Path to read the templates
  - owl-template-dest        Path to write the templates. If not specfied, STDOUT is used
```
