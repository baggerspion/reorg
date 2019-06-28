use crate::status::schema::status;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(AsChangeset, Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "status"]
pub struct Status {
    pub id: Option<i32>,
    pub title: String,
}

impl Status {
    pub fn create(nstatus: &Status, conn: &PgConnection) -> QueryResult<Status> {
        diesel::insert_into(status::table)
            .values(nstatus)
            .execute(conn)
            .expect("Error saving new reviewer");

        status::table.order(status::id.desc()).first(conn)
    }

    pub fn read(id: i32, conn: &PgConnection) -> QueryResult<Vec<Status>> {
        if id != 0 {
            status::table
                .filter(status::id.eq(id))
                .load::<Status>(conn)
        } else {
            status::table
                .load::<Status>(conn)
        }
    }

    pub fn update(id: i32, ustatus: &Status, conn: &PgConnection) -> bool {
        diesel::update(status::table.find(id))
            .set(ustatus)
            .execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(status::table.find(id)).execute(conn).is_ok()
    }
}