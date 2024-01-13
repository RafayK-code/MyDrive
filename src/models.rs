use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    #[serde(rename = "_id")]
    pub id: String,
    pub children: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(rename = "_id")]
    pub id: String,
}