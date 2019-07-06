use crate::data::DbConnection;
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};
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

fn is_valid(key: &str) -> Result<String, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(b"secret_key", Sha256::new()) {
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => match is_valid(&keys[0].to_string()) {
                Ok(_) => Outcome::Success(ApiKey(keys[0].to_string())),
                Err(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            },
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

#[post("/", format = "application/json", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>, conn: DbConnection) ->  Result<JsonValue, Status> {
    let header: Header = Default::default();
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();

    match User::by_email_and_password(email, password, &conn) {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            let claims = Registered {
                sub: Some(user.email.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token.signed(b"secret_key", Sha256::new())
                .map(|message| json!({ "success": true, "token": message }))
                .map_err(|_| Status::InternalServerError)
        }
    }
}