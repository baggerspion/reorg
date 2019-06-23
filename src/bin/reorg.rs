#![feature(proc_macro_hygiene, decl_macro)]

extern crate reorg;
extern crate rocket;

use reorg::data::create_db_pool;

fn main() {    
    let mut rocket = rocket::ignite()
        .manage(create_db_pool());
    
    rocket = reorg::conference::mount(rocket);
    rocket = reorg::review::mount(rocket);
    rocket = reorg::reviewer::mount(rocket);
    rocket = reorg::submission::mount(rocket);
    rocket = reorg::user::mount(rocket);

    rocket.launch();
}