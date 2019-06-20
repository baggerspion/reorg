#![feature(proc_macro_hygiene, decl_macro)]


extern crate reorg;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;

use diesel::prelude::*;
use diesel::sql_query;
use reorg::{DbConnection, create_db_pool};
use reorg::models::{ConfSubmission, Conference};
use rocket_contrib::json::JsonValue;

fn main() {    
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![get_conferences, 
                            get_submissions])
        .launch();
}

#[get("/")]
fn get_conferences(conn: DbConnection) -> JsonValue {
    use reorg::schema::conferences::dsl::*;
    json!(conferences.load::<Conference>(&*conn).expect("Failed to load conferences"))
}

#[get("/conference/<cid>")]
fn get_submissions(cid: i32, conn: DbConnection) -> JsonValue {
    let query = format!("SELECT S.id, S.title, U.first_name, U.last_name, round(avg(R.score),2) as score \
                 FROM Submissions S \
                 JOIN Reviews R ON S.id = R.submission_id \
                 JOIN Users U ON U.id = S.user_id \
                 WHERE S.conference_id = {} \
                 GROUP BY S.id, S.title, U.first_name, U.last_name", cid);
    json!(sql_query(&query).load::<ConfSubmission>(&*conn).expect("Failed to load submission data"))
}