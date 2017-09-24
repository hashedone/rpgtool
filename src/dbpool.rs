use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool(url: String) -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::new(config, manager).unwrap()
}

pub struct DbConn(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = req.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
