use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{request_id, State};
use std::io;

use diesel::connection::Connection;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use std::sync::Arc;
use std::sync::RwLock;

pub struct DBConnectionMiddleware {
    pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>,
}

impl DBConnectionMiddleware {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        let manager = ConnectionManager::new(database_url);

        DBConnectionMiddleware {
            pool: Arc::new(RwLock::new(
                Pool::builder().max_size(15).build(manager).unwrap(),
            )),
        }
    }
}

impl NewMiddleware for DBConnectionMiddleware {
    type Instance = DBConnectionMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(DBConnectionMiddleware {
            pool: self.pool.clone(),
        })
    }
}

impl Middleware for DBConnectionMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        state.put(DBConnection {
            connection: self.pool.read().unwrap().get().unwrap(),
        });
        Box::new(chain(state))
    }
}

#[derive(StateData)]
pub struct DBConnection {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}
