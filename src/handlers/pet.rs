use gotham::state::State;

use model::pet::Pet;
use router::*;

pub fn get(state: State) -> (State, Pet) {
    let pet = {
        let id_extractor = state.borrow::<IdExtractor>();
        println!("{:?}", id_extractor);

        Pet {
            species: "Goat".to_string(),
            name: "Sacla".to_string(),
        }
    };
    (state, pet)
}
