use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::{Form, Request}, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo}};
use super::models;

/////////////// category //////////////////////
#[post("/category?<name>&<icon>&<id>&<sort>&<descript>")]
pub fn add_category(token_user: TokenUser, name: String, icon: String, id : Option<i32>, sort: i32, descript: String, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let new_category = NewCategory {
            descript,
            icon,
            url: "".to_string(),
            label: name.clone(),
            name,
            showIndex: true,
            isDelete: false,
            sort,
            pid: None,
        };
        if !logic::update_category_by_id(id, new_category.clone(), &conn){ 
            logic::create_category(new_category, &conn);
        }
        ApiResponse {
            json: json!(get_ok_resp("")),
            status: Status::Accepted
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/category/list")]
pub fn get_category(token_user: TokenUser, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let resp = match logic::get_all_category(&conn) {
            Some(categorys) => {
                Some(models::CategoryResp::from_muti(categorys, &conn))
            }
            None => {
                None
            }
        };
        ApiResponse {
            json: json!(get_ok_resp(resp)),
            status: Status::Accepted
        }
        
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/category/list")]
pub fn get_user_category(conn: DbConn) -> ApiResponse {
    let resp = match logic::get_all_category(&conn) {
        Some(categorys) => {
            Some(models::CategoryResp::from_muti(categorys, &conn))
        }
        None => {
            None
        }
    };
    ApiResponse {
        json: json!(get_ok_resp(resp)),
        status: Status::Accepted
    }
}

#[delete("/category?<id>")]
pub fn delete_category(token_user: TokenUser, id: i32, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        logic::delete_category_by_id(id, &conn);
        ApiResponse {
            json: json!(get_ok_resp("ok")),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!("无权限"),
            status: Status::Forbidden
        }
    }
}

/////////////// goods //////////////////////
#[derive(Serialize, Deserialize, FromForm, Default)]
pub struct GoodsForm {
    pub name: String,
    pub descript: String,
    pub gallery: String,
    pub pic: String,
    pub detail: String,
    pub price: i32,
    pub stock: i32,
    pub idCategory: i32,
    pub isHot: bool,
    pub isNew: bool,
    pub id: Option<i32>
}

#[post("/goods", data = "<goods>")]
pub fn add_goods(token_user: TokenUser, goods: Json<GoodsForm>, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let new_goods = models::NewGoods {
            name: goods.name.clone(),
            descript: goods.descript.clone(),
            gallery: goods.gallery.clone(),
            pic: goods.pic.clone(),
            detail: goods.detail.clone(),
            price: goods.price,
            stock: goods.stock,
            idCategory: goods.idCategory,
            isOnSale: false,
            isHot: goods.isHot,
            isNew: goods.isNew,
        };
        if !logic::update_goods_by_id(goods.id, new_goods.clone(), &conn) {
            logic::create_goods(new_goods, &conn);
        }
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

#[get("/goods/list?<page>&<limit>&<name>")]
pub fn get_goods_admin(token_user: TokenUser, page: i32, limit: i32, name: Option<String>, conn: DbConn) -> ApiResponse {
    if let Some((goods, sum)) = logic::get_goods_resp_by_page(page, limit, name, &conn) {
        let goods_list = models::GoodsList {
            records: goods,
            current: page,
            limit,
            offset: limit,
            pages: page,
            searchCount: true,
            size: limit,
            total: sum,
        };
        ApiResponse {
            json: json!(get_ok_resp(goods_list)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(get_ok_resp("")),
            status: Status::BadRequest
        }
    }
}

#[get("/goods/queryGoods?<page>&<limit>&<idCategory>")]
pub fn get_goods_user(page: i32, limit: i32, idCategory: i32, conn: DbConn) -> ApiResponse {
    if let Some((goods, sum)) = logic::get_category_goods_resp_by_page(page, limit, idCategory, &conn) {
        let goods_list = models::GoodsList {
            records: goods,
            current: page,
            limit,
            offset: limit,
            pages: page,
            searchCount: true,
            size: limit,
            total: sum,
        };
        ApiResponse {
            json: json!(get_ok_resp(goods_list)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(get_ok_resp("")),
            status: Status::BadRequest
        }
    }
}

#[post("/goods/changeIsOnSale?<id>&<isOnSale>")]
pub fn change_onsale(token_user: TokenUser, id: i32, isOnSale: bool, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        logic::change_goods_onsale(id, isOnSale, &conn);
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

#[get("/goods?<id>")]
pub fn get_good(token_user: TokenUser, id: i32, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let goods = logic::get_goods_by_id(id, &conn).unwrap_or_default();
        let goods_info = GoodsAdminInfo {
            goods: models::GoodsResp::from_goods(goods, &conn),
            skuList: vec![]
        };
        ApiResponse {
            json: json!(get_ok_resp(goods_info)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/goods/searchHot")]
pub fn get_hot(conn: DbConn) -> ApiResponse {
    let goods = logic::get_hot_goods(&conn).unwrap_or_default();
    ApiResponse {
        json: json!(get_ok_resp(goods)),
        status: Status::Ok
    }
}

#[get("/goods/<id>")]
pub fn get_good_info(id: i32, conn: DbConn) -> ApiResponse {
    let goods = logic::get_goods_by_id(id, &conn).unwrap_or_default();
    let goods_info = GoodsInfo::from_goods(goods, &conn);
    ApiResponse {
        json: json!(get_ok_resp(goods_info)),
        status: Status::Ok
    }
}
