extern crate rustweb;
// extern crate diesel;
extern crate hyper;
// extern crate rustc_serialize;


// use self::rustweb::*;
// use self::rustweb::models::*;
// use self::diesel::prelude::*;

use hyper::{Client};
use std::io::Read;
// use rustc_serialize::Decodable;
// use rustc_serialize::Encodable;
// use rustc_serialize::json::{self, Encoder};

fn main() {
    let response = get_content("http://jsonplaceholder.typicode.com/posts/1").unwrap();
    println!("{:?}", response);
}

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buffer = String::new();
    try!(response.read_to_string(&mut buffer));
    Ok(buffer)
}
