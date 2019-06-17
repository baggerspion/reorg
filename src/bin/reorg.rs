#![feature(proc_macro_hygiene, decl_macro)]

extern crate reorg;
#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use reorg::*;
use rocket_contrib::templates::Template;
use tera::Context;

fn main() {
    let connection = create_db_pool().get().unwrap();
    
    rocket::ignite().mount("/", routes![get_conferences]).launch();
}

#[get("/")]
fn get_conferences() -> Template {
    let mut context = Context::new();

    context.insert("known_conferences", &"test");
    Template::render("layout", &context)
}