use crate::{database::conn::DbConn, schema::{order::{self, dsl}, goods}, jwt::JWT};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Prapare {
    pub addr: crate::admin::account::models::Addr,
    pub list: Vec<crate::admin::cart::models::CartResp>
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Default, Clone)]
#[table_name = "order"]
pub struct Order {
    pub id: i32,
    pub idAddress: i32,
    pub idUser: i32,
    pub payId: Option<i32>,
    pub payStatus: i32,
    pub status: i32,
    pub date: String,
    pub time: String,
    pub msg: String,
}

#[derive(Insertable, Serialize, Deserialize, Default, Clone)]
#[table_name = "order"]
pub struct NewOrder {
    pub idAddress: i32,
    pub idUser: i32,
    pub payId: Option<i32>,
    pub payStatus: i32,
    pub status: i32,
    pub date: String,
    pub time: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrderList {
    pub records: Vec<OrderResp>,
    pub current: i32,
    pub limit: i32,
    pub offset: i32,
    pub pages: i32,
    pub searchCount: bool,
    pub size: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OrderResp {
    pub consignee: String,// name
    pub consigneeAddress: String,
    pub createTime: String,
    pub id: i32,
    pub idAddress: i32,
    pub idUser: i32,
    pub items: Vec<crate::admin::cart::models::CartResp>,
    pub message: String,
    pub mobile: String,
    pub modifyTime: String,
    pub orderSn: String,
    pub payId: String,
    pub payStatus: i32,
    pub payStatusName: String,
    pub realPrice: i32,
    pub status: i32,
    pub statusName: String,
    pub totalPrice: i32,
    pub user: crate::admin::account::models::ShopUser,
    pub date: String,
    pub time: String,
    pub staff_phone: String,
}

impl OrderResp {
    pub fn from_order(order: Order, conn: &DbConn) -> Self {
        let addr = crate::admin::account::logic::get_addr_by_id(order.idAddress, conn).unwrap();
        let carts = crate::admin::cart::logic::get_cart_by_order(order.id, conn).unwrap();
        let user = crate::admin::account::logic::get_shop_user_by_id(order.idUser, conn).unwrap();
        let carts_resp = crate::admin::cart::logic::get_cart_resp(carts, order.idUser, conn);
        let mut total_price = 0;
        for cart in &carts_resp {
            total_price = total_price + cart.totalPrice;
        }
        let statusName = match order.status {
            1 => {
                "待付款".to_string()
            }
            2 => {
                "待服务".to_string()
            }
            3 => {
                "服务中".to_string()
            }
            4 => {
                "已完成".to_string()
            }
            5 => {
                "已取消".to_string()
            }
            _ => {
                "其他".to_string()
            }
        };

        // payId is service staff
        let mut payId = String::new();
        let mut staff_phone = String::new();
        if let Some(staff_if) = order.payId {
            let staff = crate::admin::account::logic::get_staff_by_id(staff_if, conn).unwrap_or_default();
            let name = staff.name;
            let phone = staff.phone;
            payId.push_str(&name);
            staff_phone.push_str(&phone);
        }
        let mut all_addr = addr.province;
        all_addr.push_str(&addr.city);
        all_addr.push_str(&addr.district);
        all_addr.push_str(&addr.addressDetail);
        Self {
            consignee: user.nickName.clone(),
            consigneeAddress: all_addr,
            createTime: "".to_string(),
            id: order.id,
            idAddress: order.idAddress,
            idUser: order.idUser,
            items: carts_resp,
            message: order.msg.clone(),
            mobile: user.mobile.clone(),
            modifyTime: "".to_string(),
            orderSn: order.id.to_string(),
            payId,
            payStatus: order.payStatus,
            payStatusName: "".to_string(),
            realPrice: total_price,
            status: order.status,
            statusName,
            totalPrice: total_price,
            user,
            date: order.date.clone(),
            time: order.time.clone(),
            staff_phone,
        }
    }
}