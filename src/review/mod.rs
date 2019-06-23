pub mod model;
pub mod schema;

use data::DbConnection;
use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Review;

#[post("/", format = "application/json", data = "<review>")]
fn create(review: Json<Review>, conn: DbConnection) -> Result<JsonValue, Status> {
    let insert = Review { id: None, ..user.into_inner() };
    Review::create(insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    Review::read(id, &conn)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<review>")]
fn update(id: i32, review: Json<Review>, conn: DbConnection) -> Result<JsonValue, Status> {
    let update = Review { id: Some(id), ..conference.into_inner() };
    Json(json!({
        "success": Review::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConnection) -> JsonValue {
    Json(json!({
        "success": Review::delete(id, &conn)
    }))
}
