use crate::{database::conn::DbConn, schema::cart::{self, dsl}, jwt::JWT, shop::models::Goods};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;


#[derive(Insertable, Queryable, Serialize, Deserialize, Default)]
#[table_name = "cart"]
pub struct Cart {
    pub count: i32,
    pub idGoods: i32,
    pub idSku: Option<i32>,
}


#[derive(Serialize, Deserialize, Default)]
pub struct CartResp {
    pub count: i32,
    pub idGoods: i32,
    pub idSku: String,
}