use crate::user::schema::users;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(AsChangeset, Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(user: &User, conn: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
            .expect("Error saving new conference");

        users::table.order(users::id.desc()).first(conn)
    }

    pub fn read(id: i32, conn: &PgConnection) -> QueryResult<Vec<User>> {
        if id != 0 {
            users::table
                .filter(users::id.eq(id))
                .load::<User>(conn)
        } else {
            users::table
                .load::<User>(conn)
        }
    }

    pub fn update(id: i32, user: &User, conn: &PgConnection) -> bool {
        diesel::update(users::table.find(id))
            .set(user)
            .execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(conn).is_ok()
    }
}