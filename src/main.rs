#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(proc_macro)]
#![feature(conservative_impl_trait)]
#![feature(custom_derive)]

extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate clap;
extern crate maud;
extern crate argon2;
extern crate chrono;
extern crate rand;

mod view;
mod dbpool;
mod model;
mod routes;
mod schema;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index(frame: view::Frame) -> maud::Markup {
    frame.render()
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> std::io::Result<NamedFile> {
    NamedFile::open(Path::new("resources/").join(file))
}

struct Config {
    rocket: rocket::Config,
    dburl: String
}

fn load_config() -> Config {
    let cmdargs = clap::App::new("RPG Web Tool server")
                    .version("0.0.1")
                    .author("Bartłomiej `hashed` Kuras <bartlomiej.kuras@o2.pl>")
                    .arg(clap::Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .value_name("ENVIRONMENT")
                        .help("Rocket environment to load")
                        .takes_value(true)
                        .possible_values(&["dev", "staging", "prod"])
                        .default_value("prod"))
                    .arg(clap::Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .value_name("HOST")
                        .help("Address to listen on")
                        .takes_value(true))
                    .arg(clap::Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("PORT")
                        .help("Port to listen on")
                        .takes_value(true))
                    .arg(clap::Arg::with_name("db")
                        .long("db")
                        .value_name("DB")
                        .help("SQLite db URL")
                        .takes_value(true)
                        .default_value("db.sqlite"))
                    .get_matches();

    let env = match cmdargs.value_of("env").unwrap_or("prod")  {
        "dev" => rocket::config::Environment::Development,
        "staging" => rocket::config::Environment::Staging,
        "prod" => rocket::config::Environment::Production,
        &_ => unreachable!()
    };

    let mut rocketcfg = rocket::Config::build(env).finalize().unwrap();

    cmdargs.value_of("host")
        .map(|h| String::from(h))
        .or_else(|| std::env::var_os("HOST").map(|h| h.into_string().unwrap()))
        .map(|h| rocketcfg.address = h);

    cmdargs.value_of("port")
        .map(|p| String::from(p))
        .or_else(|| std::env::var_os("PORT").map(|p| p.into_string().unwrap()))
        .map(|p| rocketcfg.port = p.parse().unwrap());

    Config {
        rocket: rocketcfg,
        dburl: String::from(cmdargs.value_of("db").unwrap_or("db.sqlite"))
    }
}

fn main() {
    let conf = load_config();
    rocket::custom(conf.rocket, true)
        .manage(dbpool::init_pool(conf.dburl))
        .mount("/", routes![index, static_file])
        .mount("/login", routes::login::routes())
        .launch();
}
