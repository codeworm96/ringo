#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod book;

use std::sync::{Arc, Mutex};

#[get("/gossip")]
fn gossip(book: rocket::State<Arc<Mutex<book::Book>>>) -> String {
    format!("gossip, gossip {}", book.lock().unwrap().stub)
}

fn main() {
    let status = Arc::new(Mutex::new(book::Book::new()));
    rocket::ignite()
        .manage(status.clone())
        .mount("/", routes![gossip]).launch();
}
