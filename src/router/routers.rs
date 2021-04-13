use crate::database::{self, conn::DbConn, models};
use log::debug;
use rocket::{request::Request, Rocket, http::{RawStr, Cookie, Cookies, Status, ContentType}, response::{self, Redirect, status, Responder, Response}};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use super::models::*;

use crate::paste_id::PasteId;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<paste>")]
pub fn upload(paste: Json<Post>, conn: DbConn) -> Result<String, std::io::Error> {
    let id = PasteId::new(3);
    let db_post = models::NewPost{
        username: "".to_string(),
        postdata: paste.data.to_string()
    };
    let post = database::post::create_new_post(db_post, conn);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = post.id);
    Ok(url)
}

#[get("/<id>")]
pub fn retrieve(id: &RawStr, conn: DbConn) -> Json<Post> {
    let id_i32: i32 = FromStr::from_str(id.as_str()).unwrap();
    let post_data = database::post::get_post_by_id(id_i32, conn);
    let ret_data = if post_data.is_some(){
        post_data.unwrap().postdata
    } else {
        "error, id error".to_string()
    };
    Json(Post{data: ret_data})
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<NewUser>, conn: DbConn) -> status::Accepted<String> {
    let new_user = database::user::create_new_user(user.into_inner(), conn);
    status::Accepted(Some(format!("id: '{}'", new_user.id)))
}

// #[post("/login", data = "<login>")]
// pub fn login(mut cookies: Cookies, login: Json<Login>, conn: DbConn) -> Status {
//     if check_login_data(&login.username, &login.password, conn) {
//         cookies.add_private(Cookie::new("user_id", 1.to_string()));
//         Status::Accepted
//     } else {
//         Status::Forbidden
//     }
// }

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[post("/login", data = "<login>")]
pub fn login(mut cookies: Cookies, login: Json<Login>, conn: DbConn) -> ApiResponse {
    match check_login_data(&login.username, &login.password, conn) {
        Some(user) => {
            cookies.add_private(Cookie::new("user_id", 1.to_string()));
            ApiResponse {
                json: json!(user),
                status: Status::Ok
            }
        },
        None => {
            ApiResponse {
                json: json!(""),
                status: Status::BadRequest
            }
        }
        
    }
}

fn check_login_data(username: &String, password: &String, conn: DbConn) -> Option<Users> {
    let user = database::user::get_user_by_username(username, conn);
    match user {
        Some(user) => {
            if username == &user.username && password == &user.password {
                Some(user)
            }
            else {
                None
            }
        },
        None => None
    }
}