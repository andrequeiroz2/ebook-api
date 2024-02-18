use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book{
    pub id: Option<ObjectId>,
    pub title: String,
    pub genre: String,
    pub pages: u32,
    pub author: Vec<String>
}