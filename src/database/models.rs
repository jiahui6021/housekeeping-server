use crate::schema::{post, users};
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub postdata: String,
}

#[derive(Insertable)]
#[table_name = "post"]
pub struct NewPost {
    pub username: String,
    pub postdata: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub age: Option<i32>,
    pub sex: Option<bool>,
}
#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub age: Option<i32>,
    pub sex: Option<bool>,
}
