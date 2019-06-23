use diesel::prelude::*;

#[derive(Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(Conference)]
#[belongs_to(User)]
#[table_name = "submissions"]
pub struct Submission {
    pub id: Option<i32>,
    pub conference_id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

impl Submission {
    pub fn create(submission: Submission, conn: DbConnection) -> QueryResult<Submission> {
        diesel::insert_into(submissions::table)
            .values(&submission)
            .execute(&conn)?;

        submissions::table.order(submissions::id.desc()).first(&*conn)
    }

    pub fn read(id: i32, conn: DbConnection) -> QueryResult<Vec<Submission>> {
        if id != 0 {
            submissions::table
                .filter(submissions::id.eq(id))
                .load::<Submission>(&*conn)
        } else {
            submissions::table
                .load::<Submission>(&*conn)
        }
    }

    pub fn update(id: i32, submission: Submission, conn: DbConnection) -> bool {
        diesel::update(submissions::table.find(id))
            .set(&submission)
            .execute(&*conn).is_ok()
    }

    pub fn delete(id: i32, submission: Submission, conn: DbConnection) -> bool {
        diesel::delete(submissions::table.find(id))
            .set(&submission)
            .execute(&*conn).is_ok()
    }
}