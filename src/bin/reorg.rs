#![feature(proc_macro_hygiene, decl_macro)]


extern crate reorg;
#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::sql_query;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
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

#[get("/conference/<cid>")]
fn get_submissions(cid: i32, conn: DbConnection) -> Template {
    let mut context = Context::new();
    let query = format!("SELECT S.id, S.title, U.first_name, U.last_name, round(avg(R.score),2) as score \
                 FROM Submissions S \
                 JOIN Reviews R ON S.id = R.submission_id \
                 JOIN Users U ON U.id = S.user_id \
                 WHERE S.conference_id = {} \
                 GROUP BY S.id, S.title, U.first_name, U.last_name", cid);
    let result = sql_query(&query).load::<ConfSubmission>(&*conn).expect("Failed to load submission data");
    
    /*
    let subs = submissions
        .inner_join(reviews)
        .inner_join(users)
        .filter(conference_id.eq(sid))
        .select((id, title, first_name, last_name))
        .load::<ConfSubmission>(&*conn)
        .expect("Failed to load submissions");
    */

    context.insert("submissions", &result);
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