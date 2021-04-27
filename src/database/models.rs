use crate::schema::{post, users, service, user};
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub postdata: String,
    pub user: i32,
    pub service: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "post"]
pub struct NewPost {
    pub name: String,
    pub postdata: String,
    pub user: i32,
    pub service: i32,
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
#[derive(Queryable, Serialize)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub age: Option<i32>,
    pub sex: Option<bool>,
}

#[derive(Queryable, Serialize)]
pub struct Service {
    pub id: i32,
    pub province: i32,
    pub city: i32,
    pub street: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "service"]
pub struct NewService {
    pub province: i32,
    pub city: i32,
    pub street: i32,
    pub name: String,
    pub price: i32,
}

