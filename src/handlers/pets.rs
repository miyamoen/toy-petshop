use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham_serde_json_body_parser::{create_json_response, JSONBody};

use hyper::{Body, Response, StatusCode};
use mime;
use serde_json;

use model::pet::Pet;

pub fn get(state: State) -> (State, Response) {
    let pet = Pet {
        species: "Goat".to_string(),
        name: "Sacla".to_string(),
    };

    let res = create_json_response(&state, StatusCode::Ok, &vec![pet]).unwrap();

    (state, res)
}

pub fn post(state: State) -> Box<HandlerFuture> {
    // let f = state
    //     .take::<Body>()
    //     .concat2()
    //     .then(|full_body| match full_body {
    //         Ok(valid_body) => {
    //             let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
    //             println!("Body: {:?}", valid_body);
    //             println!("Body: {}", body_content);
    //             let res = create_response(&state, StatusCode::Ok, None);
    //             future::ok((state, res))
    //         }
    //         Err(e) => return future::err((state, e.into_handler_error())),
    //     });

    Box::new(state.json::<Pet>().and_then(|(state, pet)| {
        println!("{:?}", pet);
        let res = create_json_response(&state, StatusCode::Ok, &pet).unwrap();
        Ok((state, res))
    }))
}
