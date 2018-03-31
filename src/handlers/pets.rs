use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime;
use serde_json;

use model::pet::Pet;

pub fn get(state: State) -> (State, Response) {
    let pet = {
        Pet {
            species: "Goat".to_string(),
            name: "Sacla".to_string(),
        }
    };

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            serde_json::to_string(&vec![pet])
                .expect("serialized pets")
                .into_bytes(),
            mime::APPLICATION_JSON,
        )),
    );
    (state, res)
}
