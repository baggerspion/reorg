use diesel::prelude::*;

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(user: User, conn: DbConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&conn)?;

        users::table.order(users::id.desc()).first(&*conn)
    }

    pub fn read(id: i32, conn: DbConnection) -> QueryResult<Vec<User>> {
        if id != 0 {
            users::table
                .filter(users::id.eq(id))
                .load::<User>(&*conn)
        } else {
            users::table
                .load::<User>(&*conn)
        }
    }

    pub fn update(id: i32, user: User, conn: DbConnection) -> bool {
        diesel::update(users::table.find(id))
            .set(&user)
            .execute(&*conn).is_ok()
    }

    pub fn delete(id: i32, user: User, conn: DbConnection) -> bool {
        diesel::delete(users::table.find(id))
            .set(&user)
            .execute(&*conn).is_ok()
    }
}