use serde_json::json;

extern crate mongodb;

mod models;
mod database;

fn main() {
    let huh = models::FileType::FILE;

    let json = serde_json::to_string(&huh).unwrap();
    println!("{json}");

    println!("Hello, world!");
}
