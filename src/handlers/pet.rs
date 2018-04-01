use gotham::state::State;
use gotham_serde_json_body_parser::create_json_response;
use hyper::{Response, StatusCode};

use model::pet::Pet;
use router::*;

pub fn get(state: State) -> (State, Response) {
    let pet = {
        let id_extractor = state.borrow::<IdExtractor>();
        println!("{:?}", id_extractor);

        Pet {
            species: "Goat".to_string(),
            name: "Sacla".to_string(),
        }
    };
    let res = create_json_response(&state, StatusCode::Ok, &pet).unwrap();
    (state, res)
}
