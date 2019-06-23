use diesel::prelude::*;

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
    pub fn create(reviewer: Reviewer, conn: DbConnection) -> QueryResult<Reviewer> {
        diesel::insert_into(reviewers::table)
            .values(&reviewer)
            .execute(&conn)?;

        reviewers::table.order(reviewer::id.desc()).first(&*conn)
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

    pub fn delete(id: i32, reviewer: Reviewer, conn: DbConnection) -> bool {
        diesel::delete(reviewers::table.find(id))
            .set(&reviewer)
            .execute(&*conn).is_ok()
    }
}