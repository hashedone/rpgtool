use schema::sessions;
use chrono::NaiveDateTime;

#[derive(Insertable, Queryable)]
#[table_name="sessions"]
pub struct Session {
    pub id: String,
    pub user: i32,
    pub expires: NaiveDateTime,
}
