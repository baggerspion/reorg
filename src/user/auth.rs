use crate::data::DbConnection;
use chrono::{Duration, Local};
use jwt::errors::ErrorKind;
use jwt::{encode, decode, Header, Algorithm, Validation};
use rocket::{http::Status, Outcome};
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::json::{Json, JsonValue};
use super::model::{Credentials, User};

pub struct ApiKey(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

#[derive(Debug)]
pub enum UserType {
    Submitter,
    Admin,
    Reviewer,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: i64,
}

impl ApiKey {
    fn is_valid(&self) -> Result<Claims, String> {
        let secret = "secret_key";
        let validation = Validation {leeway: 60, iss: Some("Reorg".to_string()), ..Validation::default()};
        match decode::<Claims>(&self.0, secret.as_ref(), &validation) {
            Ok(claim) => Ok(claim.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err("Token is invalid".to_string()),
                ErrorKind::InvalidIssuer => Err("Issuer is invalid".to_string()),
                _ => Err("Some other error".to_string()),
            }
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => match ApiKey(keys[0].to_string()).is_valid() {
                Ok(_) => Outcome::Success(ApiKey(keys[0].to_string())),
                Err(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            },
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

#[post("/", format = "application/json", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>, conn: DbConnection) ->  Result<JsonValue, Status> {
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();

    match User::by_email_and_password(&email, &password, &conn) {
        None => {
            Err(Status::NotFound)
        },
        Some(_) => {
            let secret = "secret_key";
            let hour_from_now = Local::now() + Duration::hours(1);

            let mut header = Header::default();
            header.alg = Algorithm::HS256;
            header.typ = Some("JWT".to_owned());

            let claims = Claims {
                iss: "Reorg".to_owned(),
                sub: email.to_owned(),
                exp: hour_from_now.timestamp(),
            };     

            encode(&header, &claims, secret.as_ref())
                .map(|message| json!({ "success": true, "token": message }))
                .map_err(|_| Status::InternalServerError)
        }
    }
}