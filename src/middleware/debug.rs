use std::io;

use futures::{future, Future};

use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{request_id, State};

pub struct DebugMiddleware {}

impl NewMiddleware for DebugMiddleware {
    type Instance = DebugMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(DebugMiddleware { ..*self })
    }
}

impl Middleware for DebugMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        let f = chain(state)
            .and_then(move |(state, response)| {
                error!("miyamo response {:?}", response);
                future::ok((state, response))
            })
            .map_err(move |(state, err)| {
                error!("miyamo err response {:?}", err);
                (state, err)
            });
        Box::new(f)
    }
}
