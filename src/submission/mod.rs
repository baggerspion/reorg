pub mod model;
pub mod schema;

use crate::user::auth::ApiKey;
use rocket::{self, http::Status, Rocket};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Submission;
use super::data::DbConnection;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/submission", routes![create, read, update, delete])
}

#[post("/", format = "application/json", data = "<submission>")]
fn create(submission: Json<Submission>, key: ApiKey, conn: DbConnection) -> Result<Json<Submission>, Status> {
    let insert = Submission { id: None, ..submission.into_inner() };
    Submission::create(&insert, &conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
fn read(id: i32, key: ApiKey, conn: DbConnection) -> Result<JsonValue, Status> {
    Submission::read(id, &conn)
        .map(|item| json!(item))
        .map_err(|_| Status::NotFound)
}

#[post("/<id>", format = "application/json", data = "<submission>")]
fn update(id: i32, submission: Json<Submission>, key: ApiKey, conn: DbConnection) -> JsonValue {
    let update = Submission { id: Some(id), ..submission.into_inner() };
    json!({
        "success": Submission::update(id, &update, &conn)
    })
}

#[delete("/<id>")]
fn delete(id: i32, key: ApiKey, conn: DbConnection) -> JsonValue {
    json!({
        "success": Submission::delete(id, &conn)
    })
}
