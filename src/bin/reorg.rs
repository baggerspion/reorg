#![feature(proc_macro_hygiene, decl_macro)]

extern crate reorg;
#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::prelude::*;
use reorg::*;
use reorg::models::*;
use rocket_contrib::templates::Template;
use tera::Context;

fn main() {    
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![get_conferences, 
                            get_submission, 
                            get_submissions])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn get_conferences(conn: DbConnection) -> Template {
    use reorg::schema::conferences::dsl::*;

    let mut context = Context::new();
    let confs = conferences.load::<Conference>(&*conn).expect("Failed to load conferences");

    context.insert("conferences", &confs);
    Template::render("conferences", &context)
}

#[get("/submissions/<sid>")]
fn get_submissions(sid: i32, conn: DbConnection) -> Template {
    use reorg::schema::submissions::dsl::*;

    let mut context = Context::new();
    let subs = submissions.filter(conference_id.eq(sid))
        .load::<Submission>(&*conn)
        .expect("Failed to load submissions");

    context.insert("submissions", &subs);
    Template::render("submissions", &context)
}

#[get("/submission/<sid>")]
fn get_submission(sid: i32, conn: DbConnection) -> Template {
    use reorg::schema::reviews::dsl::{reviews, submission_id};
    use reorg::schema::submissions::dsl::{submissions, id};

    let mut context = Context::new();
    let revs = reviews.filter(submission_id.eq(sid))
        .load::<Review>(&*conn)
        .expect("Failed to load reviews");
    let text = submissions.filter(id.eq(sid))
        .load::<Submission>(&*conn)
        .expect("Failed to load submission");

    context.insert("reviews", &revs);
    context.insert("text", &text[0]);
    Template::render("submission", &context)
}