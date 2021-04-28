use crate::{database::conn::DbConn, schema::user::{self, dsl}, jwt::JWT};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;

#[derive(Insertable, Queryable, Serialize, Deserialize, Default, Clone)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub avater: String,
    pub account: String,
    pub password: String,
    pub salt: String,
    pub name: String,
    pub sex: i32,
    pub email: String,
    pub phone: String,
    pub roleid: String,
    pub deptid: String,
    pub status: i32,
    pub version: i32,
}

impl User {
    /// Create a new user and insert into database
    // pub fn new(username: String, password: String, conn: &DbConn) -> Result<Self> {
    //     // hash
    //     //let password = hash::hash(&password);
    //     let new_user = NewUser { username, password };

    //     let user = diesel::insert_into(users::table)
    //         .values(&new_user)
    //         .get_result(&**conn)?;

    //     Ok(user)
    // }

    /// Retrieve a valid user from database with given username and password if matched.
    pub fn from(username: &str, password: &str, conn: &DbConn) -> Option<Self> {
        let user = dsl::user
        .filter(dsl::account.eq(username))
        .first::<Self>(&**conn)
        .ok();
        match user {
            Some(user) => {
                if user.password == password {
                    return Some(user);
                } else {
                    return None;
                }
            }
            None => {None}
        }
    }

    pub fn from_id(id: i32, conn: &DbConn) -> Option<Self> {
        dsl::user
        .filter(dsl::id.eq(id))
        .first::<Self>(&**conn)
        .ok()
    }

    // generate new JWT token
    pub fn generate_token(&self, duration: i64, secret: &str) -> String {
        JWT::new(self.id, duration).to_token(secret).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenUser {
    pub id: i32,
}

#[derive(Debug)]
pub enum TokenError {
    PassWordError,
    Invalid,
}

/// Parse user from request header
impl<'a, 'r> FromRequest<'a, 'r> for TokenUser {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {

        // error: if missing token, return 401 Unauthorized
        let token = request.headers().get("Authorization").next();

        // verify JWT token
        let secret = "";
        // error: jwt verification failed
        let resp = match token {
            Some(token) => {
                let jwt = JWT::from_token(token, &secret);
                if let Ok(jwt) = jwt {
                    TokenUser{
                        id: jwt.user_id()
                    }
                } else {
                    return Outcome::Failure((Status::Unauthorized, TokenError::PassWordError));
                }
            }
            None => {
                return Outcome::Failure((Status::Unauthorized, TokenError::PassWordError));
            }
        };
        
        Outcome::Success(resp)
    }

}

#[derive(Serialize, Deserialize, Default)]
pub struct AdminInfo {
    pub name: String,
    pub permissions: Vec<String>,
    pub profile: User,
    pub role: String,
    pub roles: Vec<String>
}