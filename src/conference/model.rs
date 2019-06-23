use chrono::NaiveDateTime;
use diesel::prelude::*;
use self::schema::conferences;
use super::data::DbConnection;

#[derive(Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "conferences"]
pub struct Conference {
    pub id: Option<i32>,
    pub title: String,
    #[serde(with = "date_format")]
    pub start_date: NaiveDateTime,
    #[serde(with = "date_format")]
    pub end_date: NaiveDateTime,
    pub venue: String,
    pub address: String,
    pub city: String,
    pub postcode: String,
    pub country: String,
    pub cfp: String,
}

mod date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S,) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D,) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

impl Conference {
    pub fn create(conference: Conference, conn: DbConnection) -> Conference {
        diesel::insert_into(conferences::table)
            .values(conference)
            .get_result(conn)
            .expect("Error saving new conference")
    }

    pub fn read(cid: i32, conn: DbConnection) -> QueryResult<Vec<Conference>> {
        if id != 0 {
            conferences::table
                .filter(conferences::id.eq(cid))
                .load::<Conference>(&*conn)
        } else {
            conferences::table
                .load::<Conference>(&*conn)
        }
    }

    pub fn update(cid: i32, conference: Conference, conn: DbConnection) -> bool {
        diesel::update(conferences::table.find(cid))
            .set(&conference)
            .execute(&*conn).is_ok()
    }

    pub fn delete(cid: i32, conn: DbConnection) -> bool {
        diesel::delete(conferences::table.find(cid))
            .set(&conference)
            .execute(&*conn).is_ok()
    }
}