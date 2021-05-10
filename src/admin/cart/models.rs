use crate::{database::conn::DbConn, schema::cart::{self, dsl}, jwt::JWT, shop::models::GoodsResp};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;


#[derive(Queryable, Serialize, Deserialize, Default)]
pub struct Cart {
    pub id :i32,
    pub count: i32,
    pub idGoods: i32,
    pub idSku: Option<i32>,
    pub user_id: i32,
    pub order_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Default)]
#[table_name = "cart"]
pub struct NewCart {
    pub count: i32,
    pub idGoods: i32,
    pub idSku: Option<i32>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FromCart {
    pub count: i32,
    pub idGoods: i32,
    pub idSku: String,
}

impl NewCart {
    pub fn from(user_id: i32, cart: FromCart) -> Self {
        Self {
            count: cart.count,
            idGoods: cart.idGoods,
            idSku: None,
            user_id
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct CartResp {
    pub id: i32,
    pub count: i32,
    pub idGoods: i32,
    pub idSku: Option<i32>,
    pub idUser: i32,
    pub price: i32,
    pub sku: Option<String>,
    pub title: String,
    pub goods: GoodsResp,
    pub totalPrice: i32,
}
