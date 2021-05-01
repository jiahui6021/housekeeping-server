use super::models::*;
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo}};

// #[get("/list")]
// pub fn get_topic(conn: DbConn) -> ApiResponse {

// }


// #[post("/list")]
// pub fn get_topic(conn: DbConn) -> ApiResponse {
    
// }

