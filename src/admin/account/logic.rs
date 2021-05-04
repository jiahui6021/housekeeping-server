use crate::{database::conn::DbConn, schema::{shop_user::{self, dsl}, addr}, jwt::JWT};
use super::models;
use diesel::prelude::*;

pub fn create_shop_user(cart: &models::NewShopUser, conn: &DbConn) -> models::ShopUser {
    diesel::insert_into(shop_user::table)
            .values(cart)
            .execute(&**conn)
            .expect("Error saving new category");
    shop_user::table.order(shop_user::id.desc())
        .first(&**conn).unwrap()
}

pub fn get_shop_user(mobile: &String, conn: &DbConn) -> Option<models::ShopUser> {
    dsl::shop_user
        .filter(dsl::mobile.eq(mobile))
        .first::<models::ShopUser>(&**conn)
        .ok()
}

pub fn get_shop_user_by_id(id: i32, conn: &DbConn) -> Option<models::ShopUser> {
    dsl::shop_user
        .filter(dsl::id.eq(id))
        .first::<models::ShopUser>(&**conn)
        .ok()
}

pub fn create_addr(addr: &models::NewAddr, conn: &DbConn) {
    diesel::insert_into(addr::table)
            .values(addr)
            .execute(&**conn)
            .expect("Error saving new addr");
}

pub fn get_addr_by_user(user_id: i32, conn: &DbConn) -> Option<Vec<models::Addr>> {
    addr::dsl::addr
        .filter(addr::dsl::idUser.eq(user_id))
        .load::<models::Addr>(&**conn)
        .ok()
}

pub fn get_addr_by_id(addr_id: i32, conn: &DbConn) -> Option<models::Addr> {
    addr::dsl::addr
        .filter(addr::dsl::id.eq(addr_id))
        .first::<models::Addr>(&**conn)
        .ok()
}

pub fn modify_addr(id: Option<i32>, addr: models::NewAddr, conn: &DbConn) {
    diesel::update(addr::dsl::addr.filter(addr::dsl::id.eq(id.unwrap())))
    .set(addr)
    .execute(&**conn)
    .expect("Error update goods");
}