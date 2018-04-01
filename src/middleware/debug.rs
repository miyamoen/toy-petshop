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
        // debug!("[{}] pre chain", request_id(&state));
        // Do things prior to passing the request on to other middleware and the eventual Handler
        // ..
        // For example store something in State
        // state.put(MyData { my_value: "abcdefg".to_owned() });

        let f = chain(state).and_then(move |(state, response)| {
            {
                debug!("response {:?}", response);
                // debug!("[{}] post chain", request_id(&state));
                // Do things once a response has come back
                // ..
                // For example get our data back from State
                // let data = state.borrow::<MyData>().unwrap();
            }
            future::ok((state, response))
        });
        Box::new(f)
    }
}
