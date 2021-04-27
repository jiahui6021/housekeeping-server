use crate::{database::conn::DbConn, schema::user::{self, dsl}, jwt::JWT};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;

#[derive(Serialize, Deserialize, Default)]
pub struct Router {
    pub children: Vec<Router>,
    pub component: String,
    pub hidden: bool,
    pub id: i32,
    pub name: String,
    pub num: i32,
    pub parentId: i32,
    pub path: String,
}