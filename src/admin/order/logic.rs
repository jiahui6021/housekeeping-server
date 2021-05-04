use crate::{database::conn::DbConn, schema::cart::{self, dsl}, jwt::JWT, cart::models::{CartResp}};
use super::models;
use diesel::prelude::*;

pub fn get_cards_by_ids(id_carts: Vec<&str>, user_id: i32, conn: &DbConn) -> Vec<CartResp> {
    let mut carts = Vec::new();
    for id in id_carts {
        let id = std::str::FromStr::from_str(id).unwrap_or(0);
        let cart = crate::admin::cart::logic::get_cart_by_id(id, conn);
        carts.push(cart);
    }
    crate::admin::cart::logic::get_cart_resp(carts, user_id, conn)
}