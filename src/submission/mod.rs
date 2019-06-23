#![feature(proc_macro_hygiene, decl_macro)]

pub mod model;
pub mod schema;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Submission;
use super::data::DbConnection;

#[post("/", format = "application/json", data = "<submission>")]
fn create(submission: Json<Submission>, conn: DbConnection) -> Result<Json<Submission>, Status> {
    let insert = Submission { id: None, ..submission.into_inner() };
    Submission::create(&insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    Submission::read(id, &conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<submission>")]
fn update(id: i32, submission: Json<Submission>, conn: DbConnection) -> JsonValue {
    let update = Submission { id: Some(id), ..submission.into_inner() };
    json!({
        "success": Submission::update(id, &update, &conn)
    })
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConnection) -> JsonValue {
    json!({
        "success": Submission::delete(id, &conn)
    })
}
