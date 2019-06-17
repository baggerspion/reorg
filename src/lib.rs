extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use] 
extern crate serde_derive;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use self::models::*;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to create PgConnection pool")
}

pub struct DbConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

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

// Create new objects here

pub fn create_conference(conn: &PgConnection, new_conference: &NewConference) -> Conference {
    use schema::conferences;
    diesel::insert_into(conferences::table)
        .values(new_conference)
        .get_result(conn)
        .expect("Error saving new conference")
}

pub fn create_submission(conn: &PgConnection, new_submission: &NewSubmission) -> Submission {
    use schema::submissions;
    diesel::insert_into(submissions::table)
        .values(new_submission)
        .get_result(conn)
        .expect("Error saving new submission")
}

pub fn create_user(conn: &PgConnection, new_user: &NewUser) -> User {
    use schema::users;
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error saving new user")  
}

pub fn create_reviewer(conn: &PgConnection, new_reviewer: &NewReviewer) -> Reviewer {
    use schema::reviewers;
    diesel::insert_into(reviewers::table)
        .values(new_reviewer)
        .get_result(conn)
        .expect("Error saving new reviewer")  
}

pub fn create_review(conn: &PgConnection, new_review: &NewReview) -> Review {
    use schema::reviews;
    diesel::insert_into(reviews::table)
        .values(new_review)
        .get_result(conn)
        .expect("Error saving new review")      
}

// Read objects here

pub fn read_conferences(conn: &PgConnection) -> Vec<Conference> {
    use schema::conferences::dsl::*;
    conferences.load::<Conference>(conn).expect("Error loading conferences")
}

pub fn read_reviews(conn: &PgConnection, submission: &Submission) -> Vec<Review> {
    Review::belonging_to(submission)
        .load::<Review>(conn)
        .expect("Error loading reviews")
}