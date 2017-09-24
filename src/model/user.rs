#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub uname: String,
    pub passwd: String,
    pub priv_: i32
}
