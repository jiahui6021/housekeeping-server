use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

#[post("/login?<username>&<password>")]
pub fn login_admin(username: String, password: String, conn: DbConn) -> ApiResponse {
    let user = User::from(&username, &password, &conn);
    
    match user {
        Some(user) => {
            let token = user.generate_token(10000,"");
            return ApiResponse{
                json: json!(
                    get_ok_resp(Token{
                        token
                    })
                ),
                status: Status::Ok,
            };
        }
        None => {}
    }
    ApiResponse{
        json: json!(get_ok_resp(Token{token: "sdf".to_string()})),
        status: Status::Ok,
    }
}

#[get("/info")]
pub fn info_admin(token_user: TokenUser, conn: DbConn) -> ApiResponse {
    let user = User::from_id(token_user.id, &conn);
    match user {
        Some(user) => {
            let admin_info = AdminInfo {
                name: user.name.clone(),
                role: "admin".to_string(),
                roles: vec!["administrator".to_string()],
                profile: user,
                permissions: vec![]
            };
            return ApiResponse{
                json: json!(get_ok_resp(admin_info)),
                status: Status::Ok,
            }
        }
        None => {
            return ApiResponse{
                json: json!(""),
                status: Status::BadRequest,
            }
        }
    }
    
}

#[post("/loginOrReg?<mobile>&<smsCode>")]
pub fn loginOrReg(mobile: String, smsCode: String, conn: DbConn) -> ApiResponse {
    let resp = match logic::get_shop_user(&mobile, &conn) {
        Some(shop_user) => {
            ShopUserResp {
                token: shop_user.generate_token(69000, ""),
                user: shop_user
            }
        }
        None => {
            let new_user = NewShopUser {
                mobile,
                password: "123456".to_string(),
                nickName: "未命名用户".to_string(),
                avatar: "".to_string(),
                gender: "".to_string(),
            };
            let new_user = logic::create_shop_user(&new_user, &conn);
            ShopUserResp {
                token: new_user.generate_token(69000, ""),
                user: new_user
            }
        }
    };

    ApiResponse{
        json: json!(get_ok_resp(resp)),
        status: Status::Ok,
    }
}

#[get("/getInfo")]
pub fn get_user_info(token_user: TokenUser, conn: DbConn) -> ApiResponse {
    match logic::get_shop_user_by_id(token_user.id, &conn) {
        Some(shop_user) => {
            ApiResponse{
                json: json!(get_ok_resp(shop_user)),
                status: Status::Ok,
            }
        }
        None => {
            ApiResponse{
                json: json!("鉴权失败"),
                status: Status::Forbidden,
            }
        }
    }
}


