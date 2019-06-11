extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use self::models::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Insert new objects here

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