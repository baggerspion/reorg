#![feature(proc_macro_hygiene, decl_macro)]

extern crate reorg;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;

use diesel::prelude::*;
use diesel::sql_query;
use reorg::{DbConnection, create_db_pool};
use reorg::models::{ConfSubmission, Conference, Submission};
use rocket_contrib::json::JsonValue;

fn main() {    
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![get_conference,
                            get_submission_conference,
                            get_submission_details])
        .launch();
}

#[get("/conference/titles")]
fn get_conference_titles(conn: DbConnection) -> JsonValue {
    use reorg::schema::conferences::dsl::*;
    json!(
        conferences
            .select((id, title))
            .load::<(i32, String)>(&*conn)
            .expect("Error loading conference titles")
        )
}

#[get("/conference?<conf_id>")]
fn get_conference(conf_id: Option<i32>, conn: DbConnection) -> JsonValue {
    use reorg::schema::conferences::dsl::*;
    match conf_id {
        Some(x) => json!(
                        conferences.filter(id.eq(x))
                            .load::<Conference>(&*conn)
                            .expect("Failed to load conferences")
                        ),
        None => json!(
                        conferences.load::<Conference>(&*conn)
                            .expect("Failed to load conferences")
                     ),
    }
}

#[get("/submission?<sub_id>")]
fn get_submission_details(sub_id: Option<i32>, conn: DbConnection) -> JsonValue {
    let query = match sub_id {
        Some(x) => {
            format!("SELECT S.id, S.title, U.first_name, U.last_name, round(avg(R.score),2) as score \
                 FROM Submissions S \
                 JOIN Reviews R ON S.id = R.submission_id \
                 JOIN Users U ON U.id = S.user_id \
                 WHERE S.conference_id = {} \
                 GROUP BY S.id, S.title, U.first_name, U.last_name", x)
        },
        None => {
            String::from("SELECT S.id, S.title, U.first_name, U.last_name, round(avg(R.score),2) as score \
                 FROM Submissions S \
                 JOIN Reviews R ON S.id = R.submission_id \
                 JOIN Users U ON U.id = S.user_id \
                 GROUP BY S.id, S.title, U.first_name, U.last_name")
        },
    };
    
    json!(sql_query(&query).load::<ConfSubmission>(&*conn).expect("Failed to load submission data"))
}

#[get("/submission/conf?<conf_id>")]
fn get_submission_conference(conf_id: i32, conn: DbConnection) -> JsonValue {
    use reorg::schema::submissions::dsl::{conference_id, submissions};
    json!(submissions.filter(conference_id.eq(conf_id)).load::<Submission>(&*conn).expect("Error loading submissions"))
}