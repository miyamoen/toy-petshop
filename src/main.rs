extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::{Response, StatusCode};

use gotham::handler::IntoResponse;
use gotham::http::response::create_response;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;

#[derive(Serialize)]
struct Product {
    name: String,
}

impl IntoResponse for Product {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized product")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

fn get_product_handler(state: State) -> (State, Product) {
    let product = Product {
        name: "t-shirt".to_string(),
    };

    (state, product)
}
fn router() -> Router {
    build_simple_router(|route| {
        route.get("/products/t-shirt").to(get_product_handler);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
