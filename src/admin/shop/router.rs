use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo}};
use super::models;

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
