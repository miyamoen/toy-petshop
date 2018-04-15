mod boundary;
mod handlers;
mod middleware;
mod model;
mod my_diesel;
mod router;
mod schema;

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
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde_json;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;

use router::router;

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

pub fn main() {
    dotenv().ok();
    log_init();

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
