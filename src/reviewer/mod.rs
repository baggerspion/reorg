#![feature(proc_macro_hygiene, decl_macro)]

pub mod model;
pub mod schema;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Reviewer;
use super::data::DbConnection;

#[post("/", format = "application/json", data = "<reviewer>")]
fn create(reviewer: Json<Reviewer>, conn: DbConnection) -> Result<Json<Reviewer>, Status> {
    let insert = Reviewer { id: None, ..reviewer.into_inner() };
    Reviewer::create(&insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    Reviewer::read(id, &conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<reviewer>")]
fn update(id: i32, reviewer: Json<Reviewer>, conn: DbConnection) -> JsonValue {
    let update = Reviewer { id: Some(id), ..reviewer.into_inner() };
    json!({
        "success": Reviewer::update(id, &update, &conn)
    })
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConnection) -> JsonValue {
    json!({
        "success": Reviewer::delete(id, &conn)
    })
}
