use crate::{database::conn::DbConn, schema::{shop_user::{self, dsl}, addr}, jwt::JWT};
use super::models;
use diesel::prelude::*;

pub fn create_shop_user(cart: &models::NewShopUser, conn: &DbConn) -> models::ShopUser {
    diesel::insert_into(shop_user::table)
            .values(cart)
            .execute(&**conn)
            .expect("Error saving new category");
    shop_user::table.order(shop_user::id.desc())
        .first(&**conn)
        .unwrap()
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

pub fn get_shop_user_by_page(page: i32, mobile: Option<String>, limit: i32, conn: &DbConn) -> Option<(Vec<models::ShopUser>, i32)> {
    let all_goods = match mobile {
        Some(title) => {
            dsl::shop_user
            .filter(dsl::mobile.eq(title))
            .load::<models::ShopUser>(&**conn)
            .ok()
        }
        None => {
            dsl::shop_user
            .load::<models::ShopUser>(&**conn)
            .ok()
        }
    };
    get_limit_shop_user_resp(all_goods, page, limit, conn)
}

fn get_limit_shop_user_resp(all_goods: Option<Vec<models::ShopUser>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::ShopUser>, i32)> {
    let start = (page - 1) * limit;
    let end = start + limit -1;
    let mut resp = Vec::new();
    match all_goods {
        Some(all_goods) => {
            for index in start..end {
                if let Some(good) = all_goods.get(index as usize){
                    resp.push(good.clone());
                }
            }
            Some((resp, all_goods.len() as i32))
        }
        None => {
            None
        }
    }
}

pub fn get_user_addr_by_page(page: i32, disabled: Option<bool>, limit: i32, user_id: i32, conn: &DbConn) -> Option<(Vec<models::Addr>, i32)> {
    let all_goods = match disabled {
        Some(title) => {
            // topic::dsl::topic
            // .filter(topic::dsl::disabled.eq(title))
            // .load::<models::Topic>(&**conn)
            // .ok()
            addr::dsl::addr
            .filter(addr::dsl::idUser.eq(user_id))
            .load::<models::Addr>(&**conn)
            .ok()
        }
        None => {
            addr::dsl::addr
            .filter(addr::dsl::idUser.eq(user_id))
            .load::<models::Addr>(&**conn)
            .ok()
        }
    };
    get_limit_addr_resp(all_goods, page, limit, conn)
}

fn get_limit_addr_resp(all_goods: Option<Vec<models::Addr>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::Addr>, i32)> {
    let start = (page - 1) * limit;
    let end = start + limit -1;
    let mut resp = Vec::new();
    match all_goods {
        Some(all_goods) => {
            for index in start..end {
                if let Some(good) = all_goods.get(index as usize){
                    resp.push(good.clone());
                }
            }
            Some((resp, all_goods.len() as i32))
        }
        None => {
            None
        }
    }
}

pub fn update_user_name(user_id: i32, name: String, conn: &DbConn) {
    diesel::update(shop_user::dsl::shop_user.filter(shop_user::dsl::id.eq(user_id)))
    .set(shop_user::dsl::nickName.eq(name))
    .execute(&**conn)
    .expect("Error update user name");
}

pub fn update_user_sex(user_id: i32, sex: String, conn: &DbConn) {
    diesel::update(shop_user::dsl::shop_user.filter(shop_user::dsl::id.eq(user_id)))
    .set(shop_user::dsl::gender.eq(sex))
    .execute(&**conn)
    .expect("Error update user sex");
}

pub fn update_user_pass(user_id: i32, pass: String, conn: &DbConn) {
    diesel::update(shop_user::dsl::shop_user.filter(shop_user::dsl::id.eq(user_id)))
    .set(shop_user::dsl::password.eq(pass))
    .execute(&**conn)
    .expect("Error update user pass");
}

pub fn get_shop_user_num(conn: &DbConn) -> i32 {
    use crate::schema::shop_user::dsl::*;
    use diesel::dsl;
    shop_user.select(dsl::count_star()).first::<i64>(&**conn).unwrap() as i32
}