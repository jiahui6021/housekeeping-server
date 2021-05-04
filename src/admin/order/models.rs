use crate::{database::conn::DbConn, schema::{category::{self, dsl}, goods}, jwt::JWT};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Prapare {
    pub addr: crate::admin::account::models::Addr,
    pub list: Vec<crate::admin::cart::models::CartResp>
}