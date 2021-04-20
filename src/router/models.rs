use serde::{Deserialize, Serialize};
pub use crate::database::models::{NewUser, Users};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct FeedItem {
    pub id: i32,
    pub name: String,
    pub price: String,
    pub tab: i32,
    pub image_key: String,
    pub position: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub feed_item: Vec<FeedItem>,
    pub last_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Discuss {
    pub id: i32,
    pub title: String,
    pub data: String,
    pub like: i32,
    pub service: i32,
    pub position: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub position: i32,
}

