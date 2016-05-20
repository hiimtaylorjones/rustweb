#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Post, NewPost};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str, user_id: i32) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
        user_id: user_id,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}
