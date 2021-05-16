use crate::{database::conn::DbConn, schema::order::{self, dsl}, jwt::JWT, cart::models::{CartResp}};
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

pub fn create_order(order: models::NewOrder, conn: &DbConn) -> i32 {
    diesel::insert_into(order::table)
            .values(&order)
            .execute(&**conn)
            .expect("Error saving new order");
    order::table.order(order::id.desc())
            .first::<models::Order>(&**conn)
            .unwrap().id
}

pub fn create_new_order(id_carts: Vec<i32>, id_addr: i32, date: String, time: String, id_user: i32,  conn: &DbConn) -> models::OrderResp {
    let new_order = models::NewOrder {
        idAddress: id_addr,
        idUser: id_user,
        payId: None,
        payStatus: 1,
        status: 1,
        date,
        time,
        msg: "".to_string(),
    };
    let id = create_order(new_order, conn);
    for id_cart in id_carts {
        crate::admin::cart::logic::add_order_id(id_cart, id, conn);
    }
    get_order_resp_by_id(id, conn)
}

pub fn get_order_by_range(page: i32, limit: i32, status: Option<String>, mobile: Option<String>, orderSn:Option<i32>, conn: &DbConn) -> Option<(Vec<models::OrderResp>, i32)> {
    let mut query = order::table.into_boxed();
    if let Some(status) = status {
        let status = match &status[..] {
            "unPay" => 1,
            "unSend" => 2,
            "sended" => 3,
            "finished" => 4,
            "cancel" => 5,
            "refundIng" => 6,
            "refund" => 7,
            _ => 0
        };
        if status != 0 {
            query = query.filter(order::status.eq(status));
        }
    };
    if let Some(mobile) = mobile {
        if mobile.len() > 0 {
            let id_user = crate::admin::account::logic::get_shop_user(&mobile, conn).unwrap().id;
            query = query.filter(order::idUser.eq(id_user));
        }
    }
    if let Some(order_id) = orderSn {
        query = query.filter(order::id.eq(order_id));
    }
    
    let all_goods = query
                    .load::<models::Order>(&**conn)
                    .ok();
    get_limit_order_resp(all_goods, page, limit, conn)
}

pub fn get_order_by_range_i32(user_id: i32, status: Option<i32>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::OrderResp>, i32)> {
    let mut query = order::table.into_boxed();
    if let Some(status) = status {
        if status != 0 {
            query = query.filter(dsl::status.eq(status));
        }
    }
    query = query.filter(dsl::idUser.eq(user_id));
    let all_goods = query
    .load::<models::Order>(&**conn)
    .ok();
    get_limit_order_resp(all_goods, page, limit, conn)
}

pub fn get_order_resp_by_id(id: i32, conn: &DbConn) -> models::OrderResp {
    let order = dsl::order
    .filter(dsl::id.eq(id))
    .first::<models::Order>(&**conn)
    .unwrap();
    models::OrderResp::from_order(order, conn)
}

pub fn get_order_by_user(id: i32, conn: &DbConn) -> Vec<models::Order> {
    let order = dsl::order
    .filter(dsl::idUser.eq(id))
    .load::<models::Order>(&**conn)
    .unwrap();
    order
}

fn get_limit_order_resp(all_goods: Option<Vec<models::Order>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::OrderResp>, i32)> {
    let start = (page - 1) * limit;
    let end = start + limit -1;
    let mut resp = Vec::new();
    match all_goods {
        Some(all_goods) => {
            for index in start..end {
                if let Some(good) = all_goods.get(index as usize){
                    resp.push(models::OrderResp::from_order(good.clone(),conn));
                }
            }
            resp.reverse();
            Some((resp, all_goods.len() as i32))
        }
        None => {
            None
        }
    }
    
}

pub fn update_order_pay(orderSn: i32, conn: &DbConn) {
    diesel::update(dsl::order.filter(dsl::id.eq(orderSn)))
    .set(dsl::status.eq(2))
    .execute(&**conn)
    .expect("Error update goods");
}

pub fn update_order_status(orderSn: i32, status: i32, conn: &DbConn) {
    diesel::update(dsl::order.filter(dsl::id.eq(orderSn)))
    .set(dsl::status.eq(status))
    .execute(&**conn)
    .expect("Error update goods");
}

pub fn update_order_staff(orderSn: i32, staff: Option<i32>, conn: &DbConn) {
    if let Some(staff) = staff {
        diesel::update(dsl::order.filter(dsl::id.eq(orderSn)))
        .set(dsl::payId.eq(staff))
        .execute(&**conn)
        .expect("Error update goods");
    }
}

pub fn update_order_msg(orderSn: i32, msg: String, conn: &DbConn) {
    diesel::update(dsl::order.filter(dsl::id.eq(orderSn)))
    .set(dsl::msg.eq(msg))
    .execute(&**conn)
    .expect("Error update order msg");
}

pub fn get_order_num(conn: &DbConn) -> i32 {
    use crate::schema::order::dsl::*;
    use diesel::dsl;
    order.select(dsl::count_star()).first::<i64>(&**conn).unwrap() as i32
}

pub fn get_order_sum_price(conn: &DbConn) -> i32 {
    let orders = dsl::order
    .load::<models::Order>(&**conn)
    .unwrap();
    let mut resp = 0;
    for order in orders {
        let order_resp = models::OrderResp::from_order(order, conn);
        resp = resp + order_resp.totalPrice;
    }
    resp
}


