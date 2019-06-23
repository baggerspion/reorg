pub mod model;
pub mod schema;

use data::DbConnection;
use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::User;

#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<User>, conn: DbConnection) -> Result<JsonValue, Status> {
    let insert = User { id: None, ..user.into_inner() };
    User::create(insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, conn: DbConnection) -> Result<JsonValue, Status> {
    User::read(id, &conn)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<user>")]
fn update(id: i32, user: Json<User>, conn: DbConnection) -> Result<JsonValue, Status> {
    let update = User { id: Some(id), ..user.into_inner() };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConnection) -> JsonValue {
    Json(json!({
        "success": User::delete(id, &conn)
    }))
}
