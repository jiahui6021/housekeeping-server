use crate::{database::conn::DbConn, schema::shop_user::{self, dsl}, jwt::JWT};
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

