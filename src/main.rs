mod handlers;
mod middleware;
mod model;
mod router;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

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

use router::router;

pub fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
