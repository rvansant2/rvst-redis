# rvst-redis
A Rust integration with Redis

## Motivation
Took the opportunity to get acclimated with Rust and looked at implementing a module library to interface with Redis. This library implemnts standard `Get`, `Set` and `Delete` of values, with the optional time to live to be set on the value.

## Getting started
- Make sure you have Rust installed or use the `docker-compose.yml` pointing to the `Dockerfile.production` build of the container. This is a `release` build and will take some time to stand up.
- Localize development may be faster, with `cargo watch`, you just need to install `cargo install cargo-watch` and then run `cargo watch -x run`.

## Running
- Running locally: run `cargo watch -x run`.
  - If you run `docker-compose up` for only the `Redis` service, make sure the `REDIS_CONNECTION_STRING` is set to `redis://localhost:6379/`
- Running via `docker`: run `docker-compose up` with the optional `-d` flag to run as a daemon.
  - If you run `docker-compose up` for `Redis` and `API` services, make sure the `REDIS_CONNECTION_STRING` is set to `redis://redis:6379/`

## Testing
- Import Postman contents to test the `warp` web server framework defined endpoints.

## Todos
- Modularize code, modual files were started - debugging needed.
- Fix Rust warp testing.
- Relook at the `Dockerfile.dev` and `docker-compose.yml` for the container connections to see why this isn't resolving to `localhost:8000`, though the code is building and executing properly.
