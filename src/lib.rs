#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod conference;
pub mod data;
pub mod queries;
pub mod review;
pub mod reviewer;
pub mod submission;
pub mod user;