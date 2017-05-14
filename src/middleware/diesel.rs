//!
//! Middleware for Diesel connection.
//!

use std::sync::Arc;
use diesel::Connection;
use iron::{self, Request, IronResult, BeforeMiddleware};
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

///
pub struct Value<Conn: 'static + Connection>(Arc<Pool<ConnectionManager<Conn>>>);


///
pub struct DieselMiddleware<Conn: 'static + Connection> {
    ///
    pub pool: Arc<Pool<ConnectionManager<Conn>>>,
}

impl<Conn: Connection> DieselMiddleware<Conn> {
    ///
    pub fn new(addr: &str) -> Self {
        let manager = ConnectionManager::<Conn>::new(addr);
        let pool = Pool::new(Default::default(), manager).unwrap();
        DieselMiddleware { pool: Arc::new(pool) }
    }
}

impl<Conn: Connection> Clone for DieselMiddleware<Conn> {
    fn clone(&self) -> Self {
        DieselMiddleware { pool: self.pool.clone() }
    }
}

impl<Conn: Connection> iron::typemap::Key for DieselMiddleware<Conn> {
    type Value = Value<Conn>;
}

impl<Conn: Connection> BeforeMiddleware for DieselMiddleware<Conn> {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Self>(Value(self.pool.clone()));
        Ok(())
    }
}


pub trait DieselMiddlewareExt {
    type Conn: 'static + Connection;
}

impl<Conn: 'static + Connection> DieselMiddlewareExt for DieselMiddleware<Conn> {
    type Conn = Conn;
}


///
pub trait DieselReqExt {
    ///
    fn db_conn<Conn: 'static + Connection>(&self) -> PooledConnection<ConnectionManager<Conn>>;
}

impl<'a, 'b> DieselReqExt for Request<'a, 'b> {
    fn db_conn<Conn: 'static + Connection>(&self) -> PooledConnection<ConnectionManager<Conn>> {
        let pool_value = self.extensions.get::<DieselMiddleware<Conn>>().unwrap();
        let &Value(ref pool) = pool_value;
        pool.get().unwrap()
    }
}


#[cfg(feature = "sqlite")]
pub type DieselSqliteMiddleware = DieselMiddleware<::diesel::sqlite::SqliteConnection>;

#[cfg(feature = "postgres")]
pub type DieselPgMiddleware = DieselMiddleware<::diesel::pg::PgConnection>;

#[cfg(feature = "mysql")]
pub type DieselMysqlMiddleware = DieselMiddleware<::diesel::mysql::MysqlConnection>;
