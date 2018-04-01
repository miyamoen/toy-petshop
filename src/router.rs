use gotham::router::Router;
use gotham::router::builder::*;
use handlers::*;

pub fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/pets/:id")
            .with_path_extractor::<IdExtractor>()
            .to(pet::get);
        route.associate("/pets", |assoc| {
            assoc.get().to(pets::get);
            assoc.post().to(pets::post);
        })
    })
}

#[derive(Debug, Deserialize, StateData, StaticResponseExtender)]
pub struct IdExtractor {
    pub id: u32,
}
