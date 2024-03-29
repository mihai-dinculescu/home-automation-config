# Overview
Config API for [home-automation-thermostat](https://github.com/mihai-dinculescu/home-automation-thermostat). It also interacts with [home-automation-monitoring](https://github.com/mihai-dinculescu/home-automation-monitoring).

Based on [rust-graphql-actix-juniper-diesel-example](https://github.com/mihai-dinculescu/rust-graphql-actix-juniper-diesel-example).

# Setup
## Rust & Cargo
Install `rust` and `cargo` via [rustup](https://rustup.rs). The stable version is OK.

Optional: Cargo Watch (not required, but it speeds up development greatly)
```
cargo install cargo-watch
```

## Diesel CLI
```
cargo install diesel_cli --no-default-features --features postgres
```

## Databases
```
createdb -U postgres config
createdb -U postgres config_test
```

# Run locally
Access to a postgres instance is required.
`
```
cargo run
```
or
```
cargo watch -x run
```

Open http://localhost:8080/playground.

# Run Integration tests
```
cargo test
```

# Run in Docker
The following environment variables are required in an `.env` file:

```
# general
PORT=

# postgres
POSTGRES_DB=
POSTGRES_USER=
POSTGRES_PASSWORD=

# influx
INFLUXDB_HOST=
INFLUXDB_DB=
INFLUXDB_USERNAME=
INFLUXDB_PASSWORD=

# security
API_KEY=
```

```
docker volume create --name=home-automation-config-storage
docker-compose up
```

Open [http://localhost:8080/playground](http://localhost:8080/playground).
