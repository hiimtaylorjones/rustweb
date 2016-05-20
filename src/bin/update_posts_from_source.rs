extern crate rustweb;
extern crate diesel;
extern crate hyper;

// use self::rustweb::*;
// use self::rustweb::models::*;
// use self::diesel::prelude::*;
use hyper::{Client};
use std::io::Read;


fn main() {
    println!("{:?}", get_content("http://jsonplaceholder.typicode.com/posts"));
}

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buffer = String::new();
    try!(response.read_to_string(&mut buffer));
    Ok(buffer)
}
