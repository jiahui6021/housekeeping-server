use crate::{database::conn::DbConn, schema::cart::{self, dsl}, jwt::JWT};
use super::models;
use diesel::prelude::*;

pub fn create_cart(cart: models::NewCart, conn: &DbConn) {
    diesel::insert_into(cart::table)
            .values(&cart)
            .execute(&**conn)
            .expect("Error saving new cart");
}

pub fn get_cart_by_user(user_id: i32, conn: &DbConn) -> Vec<models::Cart> {
    dsl::cart
        .filter(dsl::user_id.eq(user_id))
        .load::<models::Cart>(&**conn)
        .expect("get cart error")
}

pub fn get_cart_by_id(id: i32, conn: &DbConn) -> models::Cart {
    dsl::cart
        .filter(dsl::id.eq(id))
        .first::<models::Cart>(&**conn)
        .expect("get cart error")
}

pub fn get_cart_by_order(id: i32, conn: &DbConn) -> Option<Vec<models::Cart>> {
    dsl::cart
        .filter(dsl::order_id.eq(id))
        .load::<models::Cart>(&**conn)
        .ok()
}

pub fn update_cart_count(id: i32, count: i32, conn: &DbConn) {
    diesel::update(dsl::cart.filter(dsl::id.eq(id)))
        .set(dsl::count.eq(count))
        .execute(&**conn);
}

pub fn del_carts(mut id: String, conn: &DbConn) {
    id.remove(id.len()-1);
    id.remove(0);
    let ids: Vec<_>= id.split(',').collect();
    for id in ids {
        let id_i32 = std::str::FromStr::from_str(id).unwrap_or(0);
        diesel::delete(dsl::cart.filter(dsl::id.eq(id_i32)))
            .execute(&**conn)
            .expect("Error del cart");
    }
}

pub fn get_cart_resp(carts: Vec<models::Cart>, idUser: i32, conn: &DbConn) -> Vec<models::CartResp> {
    let mut resp = Vec::new();
    for cart in carts {
        let good = crate::shop::logic::get_goods_by_id(cart.idGoods, conn).expect("get goods error");
        let cart_resp = models::CartResp {
            id: cart.id,
            idGoods: cart.idGoods,
            idSku: None,
            idUser,
            price: good.price,
            sku: None,
            title: good.name.clone(),
            goods: crate::shop::models::GoodsResp::from_goods(good.clone(), conn),
            count: cart.count,
            totalPrice: good.price*cart.count,
        };
        resp.push(cart_resp);
    }
    //assert_eq!(resp.len(), 23);
    resp
}

pub fn add_order_id(id_cart: i32, id: i32, conn: &DbConn) {
    diesel::update(dsl::cart.filter(dsl::id.eq(id_cart)))
        .set(dsl::order_id.eq(id))
        .execute(&**conn);
}