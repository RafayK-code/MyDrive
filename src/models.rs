use bson::{Binary, Bson};
use serde::{Serialize, Deserialize};

use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i32)]
pub enum FileType{
    FILE,
    DIRECTORY,
}

impl From<FileType> for Bson {
    fn from(value: FileType) -> Self {
        match value {
            FileType::FILE => Bson::Int32(0),
            FileType::DIRECTORY => Bson::Int32(1),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(rename = "_id")]
    pub id: String, //Absolute path + file name
    pub name: String,
    pub file_type: FileType,
    pub owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Binary>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub friends: Vec<String>
}

pub struct NewFile {
    pub name: String,
    pub parent: Option<String>,
    pub data: Option<Binary>,
}