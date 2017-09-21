#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(proc_macro)]
#![feature(conservative_impl_trait)]

extern crate rocket;
extern crate clap;
extern crate maud;

mod view;

use view::{View, ViewContext};
use rocket::State;
use rocket::response::Responder;

#[get("/")]
fn index(ctx: State<ViewContext>) -> impl Responder {
    view::Frame{}.render(&ctx)
}

fn load_config() -> rocket::Config {
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
                    .get_matches();

    let env = match cmdargs.value_of("env").unwrap_or("prod")  {
        "dev" => rocket::config::Environment::Development,
        "staging" => rocket::config::Environment::Staging,
        "prod" => rocket::config::Environment::Production,
        &_ => unreachable!()
    };

    let mut config = rocket::Config::build(env).finalize().unwrap();

    cmdargs.value_of("host")
        .map(|h| String::from(h))
        .or_else(|| std::env::var_os("HOST").map(|h| h.into_string().unwrap()))
        .map(|h| config.address = h);

    cmdargs.value_of("port")
        .map(|p| String::from(p))
        .or_else(|| std::env::var_os("PORT").map(|p| p.into_string().unwrap()))
        .map(|p| config.port = p.parse().unwrap());

    config
}

fn main() {
    rocket::custom(load_config(), true)
        .attach(view::ViewContext::fairing())
        .mount("/", routes![index])
        .launch();
}
