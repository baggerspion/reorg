use chrono::NaiveDateTime;
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

#[derive(Associations, Identifiable, Queryable)]
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
}

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(Reviewer)]
#[belongs_to(Submission)]
#[table_name = "reviews"]
pub struct Review {
    pub id: i32,
    pub reviewer_id: i32,
    pub submission_id: i32,
    pub private_comments: String,
    pub shared_comments: String,
}