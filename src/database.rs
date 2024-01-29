use std::{env, io::{Error, ErrorKind}};
use bson::doc;
use dotenv::dotenv;

use mongodb::{Collection, options::{ClientOptions, ResolverConfig}, Client};

use crate::models;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
const DB_NAME: &str = "my_drive";

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
            files: client_conn.database(DB_NAME).collection("files"),
            users: client_conn.database(DB_NAME).collection("users"),
        }
    }

    pub async fn get_file(&self, file_name: &str) -> Option<models::File> {
        let filter = doc! { "_id": file_name };
        
        self.files.find_one(filter, None).await.unwrap()
    }

    async fn validate(file: Option<&models::File>, file_type: models::FileType) -> Result<(), DbError> {
        match file {
            Some(file) => {
                if file.file_type != file_type {
                    return Err(Box::new(Error::new(
                        ErrorKind::InvalidData,
                        "Cant add a file to a file! Must be a directory"
                    )))
                }
            }

            None => return Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "Directory could not be found"
            ))),
        }

        Ok(())
    }

    pub async fn add_file(&self, user_id: &str, dir_id: &str, file: models::NewFile) -> Result<(), DbError> {
        let filter = doc ! { "owner": user_id, "_id": dir_id };
        let parent_dir = self.files.find_one(filter.clone(), None).await?;

        Database::validate(parent_dir.as_ref(), models::FileType::DIRECTORY).await?;

        let new_file = models::File {
            id: dir_id.to_owned() + "/" + &file.name,
            name: file.name.to_owned(),
            file_type: models::FileType::FILE,
            owner: user_id.to_owned(),
            children: None,
            parent: Some(dir_id.to_owned()),
            data: file.data,
        };

        let update = doc! { "$push": { "children: ": &new_file.id }};

        let _ = self.files.update_one(filter, update, None).await?;
        let _ = self.files.insert_one(new_file, None).await?;

        Ok(())
    }

    pub async fn add_dir(&self, user_id: &str, dir_id: &str, dir: models::NewFile) -> Result<(), DbError> {
        let filter = doc ! { "owner": user_id, "_id": dir_id };
        let parent_dir = self.files.find_one(filter.clone(), None).await?;

        Database::validate(parent_dir.as_ref(), models::FileType::DIRECTORY).await?;

        let new_dir = models::File {
            id: dir_id.to_owned() + "/" + &dir.name,
            name: dir.name.to_owned(),
            file_type: models::FileType::DIRECTORY,
            owner: user_id.to_owned(),
            children: Some(Vec::new()),
            parent: Some(dir_id.to_owned()),
            data: None,
        };

        let update = doc! { "$push": { "children: ": &new_dir.id }};

        let _ = self.files.update_one(filter, update, None).await?;
        let _ = self.files.insert_one(new_dir, None).await?;

        Ok(())
    }

    pub async fn get_contents(&self, user_id: &str, dir_path: &str) -> Result<Vec<models::File>, DbError> {
        let filter = doc! { "owner": user_id, "_id": dir_path};

        let query = self.files.find_one(filter, None).await?;

        let dir = match query {
            Some(file) => file,

            None => return Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "Directory could not be found"
            ))),
        };

        match dir.file_type {
            models::FileType::DIRECTORY => {},

            models::FileType::FILE => return Err(Box::new(Error::new(
                ErrorKind::InvalidData,
                "Cant add a file to a file! Must be a directory"
            ))),
        }

        let mut contents = Vec::new();

        for child in dir.children.unwrap() {
            let filter = doc! { "name": child, "parent": dir_path };

            let file = self.files.find_one(filter, None).await?.unwrap();
            contents.push(file);
        }

        Ok(contents)
    }
}
