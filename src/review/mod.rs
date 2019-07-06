pub mod model;
pub mod schema;

use crate::data::DbConnection;
use crate::user::auth::ApiKey;
use rocket::{self, http::Status, Rocket};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Review;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/review", routes![create, read, update, delete])
}

#[post("/", format = "application/json", data = "<review>")]
fn create(review: Json<Review>, _key: ApiKey, conn: DbConnection) -> Result<Json<Review>, Status> {
    let insert = Review { id: None, ..review.into_inner() };
    Review::create(&insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, _key: ApiKey, conn: DbConnection) -> Result<JsonValue, Status> {
    Review::read(id, &conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<review>")]
fn update(id: i32, review: Json<Review>, _key: ApiKey, conn: DbConnection) -> JsonValue {
    let update = Review { id: Some(id), ..review.into_inner() };
    json!({
        "success": Review::update(id, &update, &conn)
    })
}

#[delete("/<id>")]
fn delete(id: i32, _key: ApiKey, conn: DbConnection) -> JsonValue {
    json!({
        "success": Review::delete(id, &conn)
    })
}
