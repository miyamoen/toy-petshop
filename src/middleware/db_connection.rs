use diesel::pg::PgConnection;
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;
use r2d2;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use std::io;
use std::sync::{Arc, RwLock};

type Pointer<T> = Arc<RwLock<T>>;

fn pointer_new<T>(t: T) -> Pointer<T> {
    Arc::new(RwLock::new(t))
}

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DBConnectionMiddleware {
    pool: Pointer<Pool>,
}

impl DBConnectionMiddleware {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        let manager = ConnectionManager::new(database_url);

        DBConnectionMiddleware {
            pool: pointer_new(Pool::builder().max_size(15).build(manager).unwrap()),
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
