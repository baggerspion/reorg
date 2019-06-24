use crate::data::DbConnection;
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

