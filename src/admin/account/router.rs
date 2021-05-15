use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp, get_err_resp}};

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

#[get("/user/info/<id>")]
pub fn get_id_user_info(id: i32, conn: DbConn) -> ApiResponse {
    match logic::get_shop_user_by_id(id, &conn) {
        Some(shop_user) => {
            let cartCount = crate::admin::cart::logic::get_cart_by_user(id, &conn).len() as i32;
            let orderCount = crate::admin::order::logic::get_order_by_user(id, &conn).len() as i32;
            let resp = ShopUserInfo {
                cartCount,
                orderCount,
                info: shop_user
            };
            ApiResponse{
                json: json!(get_ok_resp(resp)),
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


#[post("/loginByPass?<mobile>&<password>")]
pub fn login_by_pass(mobile: String, password: String, conn: DbConn) -> ApiResponse {
    match logic::get_shop_user(&mobile, &conn) {
        Some(shop_user) => {
            if shop_user.password.eq(&password) {
                let resp = ShopUserResp {
                    token: shop_user.generate_token(69000, ""),
                    user: shop_user
                };
                ApiResponse {
                    json: json!(get_ok_resp(resp)),
                    status: Status::Ok,
                }
            } else {
                ApiResponse {
                    json: json!(get_err_resp("", "账号或密码错误".to_string())),
                    status: Status::Ok
                }
            }
            
        }
        None => {
            ApiResponse {
                json: json!(get_err_resp("", "账号或密码错误".to_string())),
                status: Status::Ok
            }
        }
    }
}

#[post("/address/save", data = "<data>")]
pub fn add_addr(token_user: TokenUser, data: Json<AddrFrom>, conn: DbConn) -> ApiResponse {
    match logic::get_shop_user_by_id(token_user.id, &conn)                                                                                                                                                                                                    {
        Some(shop_user) => {
            let new_addr = NewAddr {
                idUser: token_user.id,
                addressDetail: data.addressDetail.clone(),
                areaCode: data.areaCode.clone(),
                city: data.city.clone(),
                district: data.district.clone(),
                isDefault: data.isDefault,
                name: data.name.clone(),
                postCode: data.postCode.clone(),
                province: data.province.clone(),
                tel: data.tel.clone(),
            };
            if data.id.is_some() {
                logic::modify_addr(data.id, new_addr, &conn);
            } else {
                logic::create_addr(&new_addr, &conn);
            }
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::Ok
            }
        
        }
        None => {
            ApiResponse {
                json: json!(""),
                status: Status::Forbidden
            }
        }
    }
}

#[get("/address/queryByUser")]
pub fn addr_by_user(token_user: TokenUser, conn: DbConn) -> ApiResponse {
    match logic::get_shop_user_by_id(token_user.id, &conn) {
        Some(shop_user) => {
            let resp = logic::get_addr_by_user(token_user.id, &conn).unwrap_or_default();
            ApiResponse{
                json: json!(get_ok_resp(resp)),
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

#[get("/address/<id>")]
pub fn addr_by_id(token_user: TokenUser, id: i32, conn: DbConn) -> ApiResponse {
    match logic::get_shop_user_by_id(token_user.id, &conn) {
        Some(shop_user) => {
            let resp = logic::get_addr_by_id(id, &conn).unwrap();
            ApiResponse{
                json: json!(get_ok_resp(resp)),
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

#[get("/user/list?<page>&<limit>&<mobile>")]
pub fn get_shop_user_admin(token_user: TokenUser, page: i32, limit: i32, mobile: Option<String>, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let (shop_user, num) = logic::get_shop_user_by_page(page, mobile, limit, &conn).unwrap();
        let resp = ShopUserList {
            records: shop_user,
            current: page,
            limit,
            offset: limit,
            pages: page,
            searchCount: true,
            size: limit,
            total: num,
        };
        ApiResponse {
            json: json!(get_ok_resp(resp)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[get("/address/list?<page>&<limit>&<idUser>")]
pub fn get_user_addr_admin(token_user: TokenUser, page: i32, limit: i32, idUser: i32, conn: DbConn) -> ApiResponse {
    if crate::admin::account::check_user_admin(token_user.id, &conn) {
        let (shop_user, num) = logic::get_user_addr_by_page(page, None, limit, idUser, &conn).unwrap();
        let resp = AddrList {
            records: shop_user,
            current: page,
            limit,
            offset: limit,
            pages: page,
            searchCount: true,
            size: limit,
            total: num,
        };
        ApiResponse {
            json: json!(get_ok_resp(resp)),
            status: Status::Ok
        }
    } else {
        ApiResponse {
            json: json!(""),
            status: Status::Forbidden
        }
    }
}

#[post("/logout")]
pub fn logout(conn: DbConn) -> ApiResponse {
    ApiResponse{
        json: json!(get_ok_resp("成功")),
        status: Status::Ok,
    }
}

#[post("/logout")]
pub fn logout_user(conn: DbConn) -> ApiResponse {
    ApiResponse{
        json: json!(get_ok_resp("成功")),
        status: Status::Ok,
    }
}

#[post("/updateUserName/<name>")]
pub fn update_user_name(token_user: TokenUser, name: String, conn: DbConn) -> ApiResponse {
    logic::update_user_name(token_user.id, name, &conn);
    let resp = logic::get_shop_user_by_id(token_user.id, &conn).unwrap();
    ApiResponse{
        json: json!(get_ok_resp(resp)),
        status: Status::Ok,
    }
}

#[post("/updateGender/<data>")]
pub fn update_user_sex(token_user: TokenUser, data: String, conn: DbConn) -> ApiResponse {
    logic::update_user_sex(token_user.id, data, &conn);
    let resp = logic::get_shop_user_by_id(token_user.id, &conn).unwrap();
    ApiResponse{
        json: json!(get_ok_resp(resp)),
        status: Status::Ok,
    }
}

#[post("/updatePassword/<old>/<new>/<newa>")]
pub fn update_user_pass(token_user: TokenUser, old: String, new: String, newa: String, conn: DbConn) -> ApiResponse {
    let shop_user = logic::get_shop_user_by_id(token_user.id, &conn).unwrap();
    if shop_user.password.eq(&old) {
        logic::update_user_pass(token_user.id, new, &conn);
        let resp = logic::get_shop_user_by_id(token_user.id, &conn).unwrap();
        return ApiResponse{
            json: json!(get_ok_resp(resp)),
            status: Status::Ok,
        };
    }
    ApiResponse{
        json: json!(get_ok_resp("密码错误")),
        status: Status::Forbidden,
    }
}

#[get("/dashboard")]
pub fn dashboard(token_user: TokenUser, conn: DbConn) -> ApiResponse {
    let orderCount = crate::admin::order::logic::get_order_num(&conn);
    let userCount = crate::admin::account::logic::get_shop_user_num(&conn);
    let orderSumPrice = crate::admin::order::logic::get_order_sum_price(&conn)/100;
    let likeCount = crate::admin::shop::logic::get_like_num(&conn);
    let email = vec![34,43,89,99,98,97,95];
    let resp = Dashboard {
        orderCount,
        userCount,
        orderSumPrice,
        likeCount,
        email
    };
    ApiResponse{
        json: json!(get_ok_resp(resp)),
        status: Status::Ok,
    }
}

