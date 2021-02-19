#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;
#[macro_use]
extern crate serde_json;
mod paste_id;
mod database;
mod schema;

use paste_id::PasteId;
use database::{conn::DbConn, models};
//embed_migrations!();

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket::{Rocket, http::RawStr};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;


#[derive(Serialize, Deserialize)]
struct Post {
    data: String,
}
#[post("/", data = "<paste>")]
fn upload(paste: Json<Post>, conn: DbConn) -> Result<String, std::io::Error> {
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
fn retrieve(id: &RawStr, conn: DbConn) -> Json<Post> {
    let id_i32: i32 = FromStr::from_str(id.as_str()).unwrap();
    let post_data = database::post::get_post_by_id(id_i32, conn);
    let ret_data = if post_data.is_some(){
        post_data.unwrap().postdata
    } else {
        "error, id error".to_string()
    };
    Json(Post{data: ret_data})
}

fn rocket() -> Rocket {
    rocket::ignite().attach(DbConn::fairing()).mount("/", routes![index,upload,retrieve])
}

// fn run_migration(db_con: &diesel::PgConnection){
//     embedded_migrations::run_with_output(db_con, &mut std::io::stdout())
//         .expect("migration failed.");
//     log::info!("Database migration finished.");
// }

fn main() {
    let rocket = rocket();
    //let conn = PgConn::get_one(&rocket).unwrap();
    //run_migration(&*conn);
    rocket.launch();
}