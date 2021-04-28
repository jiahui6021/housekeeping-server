use super::models::*;
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}};

#[post("/login?<username>&<password>")]
pub fn login_admin(username: String, password: String, conn: DbConn) -> ApiResponse {
    let user = User::from(&username, &password, &conn);
    #[derive(Debug, Serialize, Deserialize)]
    struct Token {
        token: String
    }
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
