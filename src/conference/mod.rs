pub mod model;
pub mod schema;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Conference;
use super::data::DbConnection;

#[post("/", format = "application/json", data = "<conference>")]
fn create(conference: Json<Conference>, conn: DbConnection) -> Result<JsonValue, Status> {
    let insert = Conference { id: None, ..conference.into_inner() };
    Conference::create(insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    Conference::read(id, &conn)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<conference>")]
fn update(id: i32, conference: Json<Conference>, conn: DbConnection) -> Result<JsonValue, Status> {
    let update = Conference { id: Some(id), ..conference.into_inner() };
    Json(json!({
        "success": Conference::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConnection) -> JsonValue {
    Json(json!({
        "success": Conference::delete(id, &conn)
    }))
}
