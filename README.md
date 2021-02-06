# Rust Simple Webserver

## How to Run
### Cargo
```shell script
$ COVID19_SERVICE_ADDRESS=https://api.covid19api.com cargo run
```
### Docker
```shell script
$ docker build -t rust-simple-webserver .
$ docker run \
  -e COVID19_SERVICE_ADDRESS="https://api.covid19api.com" \
  -p 8080:8080 \
  rust-simple-webserver
```