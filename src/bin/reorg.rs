#![feature(proc_macro_hygiene, decl_macro)]

extern crate reorg;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;

use reorg::data::{DbConnection, create_db_pool};

fn main() {    
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![get_conference,
                            get_conference_submissions_count,
                            get_conference_titles,
                            get_reviews_submission,
                            get_submission_conference,
                            get_submission_details])
        .launch();
}