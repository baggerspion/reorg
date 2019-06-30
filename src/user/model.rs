use crate::user::schema::users;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
   pub email: String,
   pub password: String,
}

#[derive(AsChangeset, Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl User {
    pub fn create(user: &User, conn: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
            .expect("Error saving new user");

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

    pub fn by_email_and_password(
        email_: String, 
        password_: String, 
        conn: &PgConnection
    ) -> Option<User> {
        let res = users::table
            .filter(users::email.eq(email_))
            .filter(users::password.eq(password_))
            .order(users::id)
            .first(conn);
        match res {
            Ok(user) => Some(user),
            Err(_) => {
                None
            }
        }
    }
}