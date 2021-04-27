use super::models::*;
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::str::FromStr;
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
    ApiResponse{
        json: json!(token_user.id),
        status: Status::Ok,
    }
}
