use diesel::prelude::*;
use self::model::User;
use self::schema::reviewers;
use super::conference::model::Conference;
use super::data::DbConnection;

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(Conference)]
#[belongs_to(User)]
#[table_name = "reviewers"]
pub struct Reviewer {
    pub id: Option<i32>,
    pub conference_id: i32,
    pub user_id: i32,
}

impl Reviewer {
    pub fn create(reviewer: Reviewer, conn: DbConnection) -> Reviewer {
        diesel::insert_into(reviewers::table)
            .values(reviewer)
            .get_result(&*conn)
            .expect("Error saving new reviewer")
    }

    pub fn read(id: i32, conn: DbConnection) -> QueryResult<Vec<Reviewer>> {
        if id != 0 {
            reviewers::table
                .filter(reviewers::id.eq(id))
                .load::<Reviewer>(&*conn)
        } else {
            reviwers::table
                .load::<Reviewer>(&*conn)
        }
    }

    pub fn update(id: i32, reviewer: Reviewer, conn: DbConnection) -> bool {
        diesel::update(reviewers::table.find(id))
            .set(&reviewer)
            .execute(&*conn).is_ok()
    }

    pub fn delete(id: i32, conn: DbConnection) -> bool {
        diesel::delete(reviewers::table.find(id))
            .set(&reviewer)
            .execute(&*conn).is_ok()
    }
}