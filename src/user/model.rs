use crate::user::schema::users;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
   pub email: String,
   pub password: String,
}

#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub roles: Option<Vec<String>>,
}

impl User {
    pub fn create(user: &User, conn: &PgConnection) -> QueryResult<User> {
        let mut hasher = Sha256::new();
        hasher.input_str(&user.password);

        let new_user = &User{
            id: None,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            password: hasher.result_str(),
            roles: None,
        };
        
        diesel::insert_into(users::table)
            .values(new_user)
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

    pub fn by_email_and_password(email_: &String, password_: &String, conn: &PgConnection) -> Option<User> {
        let mut hasher = Sha256::new();
        hasher.input_str(&password_);

        let res = users::table
            .filter(users::email.eq(email_))
            .filter(users::password.eq(hasher.result_str()))
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