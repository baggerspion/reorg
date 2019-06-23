use crate::reviewer::model::Reviewer;
use crate::review::schema::reviews;
use crate::submission::model::Submission;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(AsChangeset, Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(Reviewer)]
#[belongs_to(Submission)]
#[table_name = "reviews"]
pub struct Review {
    pub id: Option<i32>,
    pub reviewer_id: i32,
    pub submission_id: i32,
    pub private_comments: String,
    pub shared_comments: String,
    pub score: i32
}

impl Review {
    pub fn create(review: &Review, conn: &PgConnection) -> QueryResult<Review> {
        diesel::insert_into(reviews::table)
            .values(review)
            .execute(conn)
            .expect("Error saving new review");

        reviews::table.order(reviews::id.desc()).first(conn)
    }

    pub fn read(id: i32, conn: &PgConnection) -> QueryResult<Vec<Review>> {
        if id != 0 {
            reviews::table
                .filter(reviews::id.eq(id))
                .load::<Review>(conn)
        } else {
            reviews::table
                .load::<Review>(conn)
        }
    }

    pub fn update(id: i32, review: &Review, conn: &PgConnection) -> bool {
        diesel::update(reviews::table.find(id))
            .set(review)
            .execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(reviews::table.find(id)).execute(conn).is_ok()
    }
}