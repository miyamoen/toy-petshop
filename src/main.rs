mod handlers;
mod middleware;
mod model;
mod router;

extern crate simplelog;
#[macro_use]
extern crate log;
use simplelog::*;
use std::fs::File;
extern crate dotenv;

extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate gotham_serde_json_body_parser;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use std::env;

use router::router;

pub fn main() {
    log_init();
    dotenv().ok();

    let addr: &str = &env::var("ADDRES").unwrap_or("0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(7878);
    println!("Listening for requests at http://{}", addr);
    gotham::start((addr, port), router())
}

fn log_init() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create("my_rust_binary.log").unwrap(),
        ),
    ]).unwrap();
}
