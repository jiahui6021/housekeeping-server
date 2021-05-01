use crate::{database::conn::DbConn, schema::cart::{self, dsl}, jwt::JWT};
use super::models;
use diesel::prelude::*;

pub fn create_cart(id: i32, cart: models::Cart, conn: &DbConn) {
    diesel::insert_into(cart::table)
            .values(&cart)
            .execute(&**conn)
            .expect("Error saving new cart");
}
