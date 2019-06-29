use crate::conference::model::Conference;
use crate::submission::schema::submissions;
use crate::user::model::User;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde_json::Value;

#[derive(AsChangeset, Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(Conference)]
#[belongs_to(User)]
#[table_name = "submissions"]
pub struct Submission {
    pub id: Option<i32>,
    pub conference_id: i32,
    pub user_id: i32,
    pub status_id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Value>,
}

impl Submission {
    pub fn create(submission: &Submission, conn: &PgConnection) -> QueryResult<Submission> {
        diesel::insert_into(submissions::table)
            .values(submission)
            .execute(conn)
            .expect("Error saving new submission");

        submissions::table.order(submissions::id.desc()).first(conn)
    }

    pub fn read(id: i32, conn: &PgConnection) -> QueryResult<Vec<Submission>> {
        if id != 0 {
            submissions::table
                .filter(submissions::id.eq(id))
                .load::<Submission>(conn)
        } else {
            submissions::table
                .load::<Submission>(conn)
        }
    }

    pub fn update(id: i32, submission: &Submission, conn: &PgConnection) -> bool {
        diesel::update(submissions::table.find(id))
            .set(submission)
            .execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(submissions::table.find(id)).execute(conn).is_ok()
    }
}