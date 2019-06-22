use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::sql_types::{Int4, Int8, Numeric, Varchar};
use super::schema::*;

mod date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S,) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D,) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Insertable)]
#[table_name = "conferences"]
pub struct NewConference {
    pub title: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub venue: String,
    pub address: String,
    pub city: String,
    pub postcode: String,
    pub country: String,
    pub cfp: String,
}

#[derive(Deserialize, Identifiable, Queryable, Serialize)]
#[table_name = "conferences"]
pub struct Conference {
    pub id: i32,
    pub title: String,
    #[serde(with = "date_format")]
    pub start_date: NaiveDateTime,
    #[serde(with = "date_format")]
    pub end_date: NaiveDateTime,
    pub venue: String,
    pub address: String,
    pub city: String,
    pub postcode: String,
    pub country: String,
    pub cfp: String,
}

#[derive(Insertable)]
#[table_name = "submissions"]
pub struct NewSubmission {
    pub conference_id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Associations, Deserialize, Identifiable, Queryable, Serialize)]
#[belongs_to(Conference)]
#[belongs_to(User)]
#[table_name = "submissions"]
pub struct Submission {
    pub id: i32,
    pub conference_id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "reviewers"]
pub struct NewReviewer {
    pub conference_id: i32,
    pub user_id: i32,
}

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Conference)]
#[belongs_to(User)]
#[table_name = "reviewers"]
pub struct Reviewer {
    pub id: i32,
    pub conference_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "reviews"]
pub struct NewReview {
    pub reviewer_id: i32,
    pub submission_id: i32,
    pub private_comments: String,
    pub shared_comments: String,
    pub score: i32
}

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[belongs_to(Reviewer)]
#[belongs_to(Submission)]
#[table_name = "reviews"]
pub struct Review {
    pub id: i32,
    pub reviewer_id: i32,
    pub submission_id: i32,
    pub private_comments: String,
    pub shared_comments: String,
    pub score: i32
}

#[derive(Debug, Deserialize, Queryable, QueryableByName, Serialize)]
pub struct ConfSubmission {
    #[sql_type = "Int4"] pub id: i32,
    #[sql_type = "Varchar"] pub title: String,
    #[sql_type = "Varchar"] pub first_name: String,
    #[sql_type = "Varchar"] pub last_name: String,
    #[sql_type = "Numeric"] pub score: BigDecimal,
    #[sql_type = "Int8"] pub count: i64,
    #[sql_type = "Numeric"] pub stddev: BigDecimal,
}