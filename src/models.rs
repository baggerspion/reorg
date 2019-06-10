use chrono::NaiveDateTime;
use super::schema::{conferences, submissions};

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

#[derive(Queryable)]
pub struct Conference {
    pub id: i32,
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

#[derive(Insertable)]
#[table_name = "submissions"]
pub struct NewSubmission {
    pub conference_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Associations, Queryable)]
#[belongs_to(Conference)]
pub struct Submission {
    pub id: i32,
    pub conference_id: i32,
    pub title: String,
    pub content: String,
}