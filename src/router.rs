use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::router::Router;
use gotham::router::builder::*;
use handlers::*;

use middleware::debug::DebugMiddleware;

pub fn router() -> Router {
    let (chain, pipelines) = single_pipeline(new_pipeline().add(DebugMiddleware {}).build());

    build_router(chain, pipelines, |route| {
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
