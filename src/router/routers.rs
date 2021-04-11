use crate::database::{self, conn::DbConn, models};
use rocket::{Rocket, http::{RawStr, Cookie, Status}, response::Redirect};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use super::models::*;
use rocket::response::status;

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