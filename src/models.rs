#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

use super::schema::posts;

#[insertable_into(posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: i32,
}

//#[insertable_into(users)]
pub struct NewUser<'a> {
    pub name: &'a str,
}
