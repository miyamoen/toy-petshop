mod handlers;
mod middleware;
mod model;
mod my_diesel;
mod router;
mod schema;

extern crate simplelog;
#[macro_use]
extern crate log;
use my_diesel::create_post;
use simplelog::*;
use std::fs::File;
use std::io::Read;
use std::io::stdin;
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

#[macro_use]
extern crate diesel;
use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use my_diesel::establish_connection;
use router::router;

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

pub fn main() {
    log_init();
    dotenv().ok();

    use schema::posts::dsl::*;

    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut var_title = String::new();
    stdin().read_line(&mut var_title).unwrap();
    let var_title = &var_title[..(var_title.len() - 1)]; // Drop the newline character
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        var_title, EOF
    );
    let mut var_body = String::new();
    stdin().read_to_string(&mut var_body).unwrap();

    let post = create_post(&connection, var_title, &var_body);
    println!("\nSaved draft {} with id {}", var_title, post.id);

    let results = posts
        .filter(published.eq(false))
        .limit(5)
        .load::<model::Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

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
