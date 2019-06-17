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
    rocket::ignite().mount("/", routes![get_conferences])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn get_conferences() -> Template {
    use reorg::schema::conferences::dsl::*;

    let conn = create_db_pool().get().unwrap();
    let mut context = Context::new();
    let confs = conferences.load::<Conference>(&*conn).expect("Failed to load conferences");

    context.insert("conferences", &confs);
    Template::render("conferences", &context)
}