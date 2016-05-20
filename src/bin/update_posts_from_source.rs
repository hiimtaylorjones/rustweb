extern crate rustweb;
extern crate diesel;
extern crate hyper;

// use self::rustweb::*;
// use self::rustweb::models::*;
// use self::diesel::prelude::*;
use hyper::{Client};
use std::io::Read;


fn main() {
    let client = Client::new();
    let url = "http://jsonplaceholder.typicode.com/posts";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf)
}
