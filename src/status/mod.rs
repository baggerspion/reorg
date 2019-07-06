pub mod model;
pub mod schema;

use crate::data::DbConnection;
use crate::user::auth::ApiKey;
use rocket::{self, http::Status, Rocket};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Status as StatusStruct;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/status", routes![create, read, update, delete])
}

#[post("/", format = "application/json", data = "<status>")]
fn create(status: Json<StatusStruct>, key: ApiKey, conn: DbConnection) -> Result<Json<StatusStruct>, Status> {
    let insert = StatusStruct { id: None, ..status.into_inner() };
    StatusStruct::create(&insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, key: ApiKey, conn: DbConnection) -> Result<JsonValue, Status> {
    StatusStruct::read(id, &conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<status>")]
fn update(id: i32, status: Json<StatusStruct>, key: ApiKey, conn: DbConnection) -> JsonValue {
    let update = StatusStruct { id: Some(id), ..status.into_inner() };
    json!({
        "success": StatusStruct::update(id, &update, &conn)
    })
}

#[delete("/<id>")]
fn delete(id: i32, key: ApiKey, conn: DbConnection) -> JsonValue {
    json!({
        "success": StatusStruct::delete(id, &conn)
    })
}