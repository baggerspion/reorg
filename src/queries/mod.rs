use crate::data::DbConnection;
use crate::review::model::Review;
use crate::review::schema::reviews::dsl::{reviews, submission_id};
use crate::submission::model::Submission;
use crate::submission::schema::submissions::dsl::{submissions, conference_id};
use diesel;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{self, http::Status, Rocket};
use rocket_contrib::json::JsonValue;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/queries", routes![read_conf_subs])
}

/* Get the submissions to a given conference */
#[get("/conference/<cid>/submissions")]
fn read_conf_subs(cid: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    submissions
        .filter(conference_id.eq(cid))
        .load::<Submission>(&*conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

/* Get the reviews to a given submission */
#[get("/submission/<sid>/reviews")]
fn read_sub_revs(sid: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    reviews
        .filter(submission_id.eq(sid))
        .load::<Review>(&*conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

