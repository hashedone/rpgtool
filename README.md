# RPG Tool

## Requirements

To build rpg tool you need rust nightly build. To get it install [rustup](https://www.rustup.rs/)
and type `rustup install nightly`. You would also need some libraries - for now I know about
libsqlite3.

## Building
To build rpg tool you need to call `cargo +nightly build`. Then in project folder call:
`rustup override set nightly` to set nightly build as default for this project.

## DB preparing
To prepare DB environment you need to install diesel-cli first:
`cargo install diesel_cli --no-default-features --features "sqlite"`.
When its done setup your db with: `diesel setup`, and then run database migrations:
`diesel migration run`.

## Running RPG Tool

To run rpgtool in development mode call: `cargo run -- -e dev` which should run RPG tool on localhost on port 8000.
To run it in release mode you may run just `cargo run`, but you need properly configured
[`Rocket.toml`](https://rocket.rs/guide/configuration/#rockettoml) file. Remember, that host and port file would
be overrident by HOST and PORT environment variable, but you can hard set it by passing as argumenst:
`cargo run -- -e dev -h localhost -p 8000`. To list all arguments type: `cargo run -- --help`.

## Initial user

For now there is no easy way to add first user. User table has fields: `id` (unique id), `uname`, (user name),
`passwd` (argon2 encrypted password), `priv` (privilidges level). To encrypt password with argon2 you may
use [this](https://antelle.github.io/argon2-browser/) site - in db `encoded` value should be stored, so it
can be properly verified by RPG tool. Privilidges level is designed that lower number means higher privilidges,
with 0 gaining superuser privilidges - however its not used at all for now, so it may change.
