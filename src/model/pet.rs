use gotham::handler::IntoResponse;
use gotham::http::response::create_response;
use gotham::state::State;

use hyper::{Response, StatusCode};
use mime;

use serde_json;

#[derive(Serialize)]
pub struct Pet {
    pub species: String,
    pub name: String,
}

impl IntoResponse for Pet {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized pet")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}
