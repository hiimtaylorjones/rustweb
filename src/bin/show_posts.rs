extern crate rustweb;
extern crate diesel;

use self::rustweb::*;
use self::rustweb::models::*;
use self::diesel::prelude::*;

fn main() {
    use rustweb::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
