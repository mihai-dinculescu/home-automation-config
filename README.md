# Overview
Config API for [home-automation-thermostat](https://github.com/mihai-dinculescu/home-automation-thermostat).

Based on [rust-graphql-actix-juniper-diesel-example](https://github.com/mihai-dinculescu/rust-graphql-actix-juniper-diesel-example).

# Setup
Install `rust` and `cargo` via `rustup` (https://rustup.rs/). The stable version is OK.

Diesel CLI
```
cargo install diesel_cli --no-default-features --features postgres
```

Optional: Cargo Watch (not required, but it speeds up development greatly)
```
cargo install cargo-watch
```

# Run locally
Access to a postgres instance is required.

```
cargo run
```
or
```
cargo watch -x run
```

Open http://localhost:8080/playground.

# Run in Docker
```
docker volume create --name=home-automation-config-storage
docker-compose up
```

Open http://localhost:8080/playground.
