use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

pub fn create_db_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to create PgConnection pool")
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
impl Deref for DbConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}