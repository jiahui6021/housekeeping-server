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