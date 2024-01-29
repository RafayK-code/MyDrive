extern crate mongodb;

mod models;
mod database;
mod system;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = database::Database::new("MONGODB_URI").await;

    let v = db.get_contents("sentinelk", "root").await.unwrap();
    for file in v {
        println!("{:?}", file)
    }

    let _ = db.add_file("sentinelk", "root", models::NewFile { 
        name: "file3".to_owned(), 
        parent: Some("root".to_owned()), 
        data: None 
    }).await;

    Ok(())
}
