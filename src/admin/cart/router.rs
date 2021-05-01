use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo},
shop::models::Goods};

#[post("/add", data = "<add_cart>")]
pub fn add_cart(token_user: TokenUser, add_cart: Json<Cart>, conn: DbConn) -> ApiResponse {
    let cart = add_cart.into_inner();
    logic::create_cart(token_user.id, cart, &conn);
    ApiResponse {
        json: json!(get_ok_resp("")),
        status: Status::Ok
    }
}

// #[get("/queryByUser")]
// pub fn get_cart(token_user: TokenUser, conn: DbConn) -> ApiResponse {

// }



// 1 -7
// 2 - 10 -8 4
// 3 5
// 6 9
