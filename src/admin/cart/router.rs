use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::{FromForm, Request}, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo},
shop::models::Goods};

#[post("/cart/add", data = "<add_cart>")]
pub fn add_cart(token_user: TokenUser, add_cart: Json<FromCart>, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_shop_user(token_user.id, &conn) {
        let new_cart = NewCart::from(token_user.id, add_cart.into_inner());
        logic::create_cart(new_cart, &conn);
        ApiResponse {
            json: json!(get_ok_resp("")),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(get_ok_resp("")),
            status: Status::Forbidden
        }
    } 
}

#[get("/cart/queryByUser")]
pub fn get_cart(token_user: TokenUser, conn: DbConn) -> ApiResponse {
    match crate::admin::account::get_shop_user_by_id(token_user.id, &conn) {
        Some(user) => {
            let cart = logic::get_cart_by_user(token_user.id, &conn);
            let resp = logic::get_cart_resp(cart, token_user.id, &conn);
            ApiResponse {
                json: json!(get_ok_resp(resp)),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::Forbidden
            }
        }
    }
}

#[post("/cart/update/<id>/<count>")]
pub fn add_cart_count(token_user: TokenUser, id: i32, count: i32, conn: DbConn) -> ApiResponse {
    match crate::admin::account::get_shop_user_by_id(token_user.id, &conn) {
        Some(user) => {
            logic::update_cart_count(id, count, &conn);
            ApiResponse {
                json: json!(""),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::Forbidden
            }
        }
    }
}

#[delete("/cart", data = "<ids>")]
pub fn del_cart(token_user: TokenUser, mut ids: String, conn: DbConn) -> ApiResponse {
    match crate::admin::account::get_shop_user_by_id(token_user.id, &conn) {
        Some(user) => {
            logic::del_carts(ids, &conn);
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::Forbidden
            }
        }
    }
}



// 1 -7
// 2 - 10 -8 4
// 3 5
// 6 9
