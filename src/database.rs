use std::env;
use bson::doc;
use dotenv::dotenv;

use mongodb::{Collection, options::{ClientOptions, ResolverConfig}, Client};

use crate::models;

pub struct Database {
    files: Collection<models::File>,
    users: Collection<models::User>,
}

impl Database {
    pub async fn new(key: &str) -> Self {
        dotenv().ok();

        let client_uri = env::var(key).expect(format!("You must set the {key} enviornment var!").as_str());
        let options = ClientOptions::parse_with_resolver_config(client_uri, ResolverConfig::cloudflare()).await.unwrap();
        let client_conn = Client::with_options(options).unwrap();

        Database {
            files: client_conn.database("my_drive").collection("files"),
            users: client_conn.database("my_drive").collection("users"),
        }
    }

    pub async fn get_file(&self, file_name: &str) -> Option<models::File> {
        let filter = doc! { "_id": file_name };
        
        self.files.find_one(filter, None).await.unwrap()
    }

    pub async fn add_file(&self, user_id: &str, dir_name: &str, file: models::File) {
        let filter = doc ! { "_id": dir_name };
        let dir = self.files.find_one(filter.clone(), None).await.unwrap();

        match dir {
            Some(file) => {
                match file.file_type {
                    models::FileType::FILE => return,
                    models::FileType::DIRECTORY => {},
                }
            }

            None => return,
        }

        let update = doc! { "$push": { "children: ": file.id }};

        let _result = self.files.update_one(filter, update, None).await.unwrap();
    }

    pub async fn get_contents(&self, user_id: &str, dir_name: &str) -> Option<Vec<models::File>> {
        let filter = doc! { "_id": dir_name, "file_type": models::FileType::DIRECTORY };

        let query = self.files.find_one(filter, None).await.unwrap();

        let dir = match query {
            Some(file) => file,
            None => return None,
        };

        if dir.file_type != models::FileType::DIRECTORY {
            return None;
        }

        let mut contents = Vec::new();

        for child in dir.children.unwrap() {
            let filter = doc! { "_id": child };

            let file = self.files.find_one(filter, None).await.unwrap().unwrap();
            contents.push(file);
        }

        Some(contents)
    }
}