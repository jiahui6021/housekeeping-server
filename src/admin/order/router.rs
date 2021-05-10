use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::{Form, Request}, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo}};
use super::models;

#[get("/order/prepareCheckout?<chosenAddressId>&<idCarts>")]
pub fn prepare(token_user: TokenUser, chosenAddressId: Option<i32>, idCarts : String, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let addr_id = chosenAddressId.unwrap_or(1);
        let addr = crate::admin::account::logic::get_addr_by_id(addr_id, &conn).unwrap_or_default();
        let id_carts: Vec<_> = idCarts.split(',').collect();
        let carts = logic::get_cards_by_ids(id_carts, token_user.id, &conn);
        let resp = Prapare {
            addr,
            list: carts
        };
        ApiResponse {
            json: json!(get_ok_resp(resp)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[post("/order/save?<idAddress>&<message>&<idCarts>")]
pub fn save_order(token_user: TokenUser, idAddress: i32, message: String, idCarts: String, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let id_carts = crate::util::split_string_to_i32_vec(idCarts);
        logic::create_new_order(id_carts, idAddress, token_user.id, &conn);
        ApiResponse {
            json: json!(get_ok_resp("")),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/order/getOrders?<page>&<limit>&<status>")]
pub fn get_order(token_user: TokenUser, page: i32, limit: i32, status: Option<i32>, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let (order, num) = logic::get_order_by_range(page, limit, status, &conn).unwrap();
        let resp = models::OrderList {
            records: order,
            current: page,
            limit,
            offset: limit,
            pages: page,
            searchCount: true,
            size: limit,
            total: num,
        };
        ApiResponse {
            json: json!(get_ok_resp(resp)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/order/list?<page>&<limit>&<status>")]
pub fn get_order_admin(token_user: TokenUser, page: i32, limit: i32, status: Option<i32>, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let (order, num) = logic::get_order_by_range(page, limit, status, &conn).unwrap();
        let resp = models::OrderList {
            records: order,
            current: page,
            limit,
            offset: limit,
            pages: page,
            searchCount: true,
            size: limit,
            total: num,
        };
        ApiResponse {
            json: json!(get_ok_resp(resp)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/order/<id>")]
pub fn get_id_order(token_user: TokenUser, id: i32, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let order = logic::get_order_resp_by_id(id, &conn);
        ApiResponse {
            json: json!(get_ok_resp(order)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[post("/pay/wx/prepare?<orderSn>")]
pub fn pay_order(token_user: TokenUser, orderSn: i32, conn: DbConn) -> ApiResponse {
    logic::update_order_pay(orderSn, &conn);
    ApiResponse {
        json: json!(get_ok_resp("支付成功")),
        status: Status::Ok
    }
}
