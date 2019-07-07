# REORG 
## Conference Submission Management

[![Build Status](https://travis-ci.org/therealpadams/reorg.svg)](https://travis-ci.org/therealpadams/reorg)
[![GitHub license](https://img.shields.io/github/license/therealpadams/reorg.svg)](license.txt)

## Purpose
REORG is a Rust-based conference submission management system, primarilly intended as a replacement for the GRORG system used by [Write The Docs](http://www.writethedocs.org). REORG is a Rust application using Diesel (for database management) and Rocket (for HTTP request handling). REORG is purely a backend system, but a JS library will be provided for convenience, so that the system can easily be integrated into any conference website.


## Build Instructions
Using Rust nightly:
```sh
cargo build --verbose --all
cargo install diesel_cli --no-default-features --features "postgres"
```

## Getting Started
### Preparations
Ensure you have a Postgres database running and accessible, with a known user:
```sh
echo "DATABASE_URL=postgres://<username>:<password>@<server>/<db_name>" > .env
diesel setup
diesel migration run
```

### Seeding The Database
After preparing the database, it can be seeded with rnadom data:
```sh
cargo run --bin seed
```

### Running REORG
```sh
cargo run --bin reorg
```

## Using REORG
By default REORG will be listening on port 8000 of the machine it is running. Having seeded data, you can test the REORG server by, for example:

### Get an authentication key:
```sh
curl -H "Content-Type: application/json" -X POST -d '{"email": "fake@fake.com", "password": "password"}' "localhost:8000/auth"
```

### Find the submissions to a specific conference
```sh
curl -H "x-api-key: <your key here>" "localhost:8000/queries/conference/<conference id>/submissions"
```