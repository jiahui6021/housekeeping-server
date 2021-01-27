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

use rocket::{Data, Rocket, http::RawStr};
use std::{fs::File, path::Path};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;


#[derive(Serialize, Deserialize)]
struct Post {
    data: String,
}
#[post("/", data = "<paste>")]
fn upload(paste: Json<Post>, conn: DbConn) -> Result<String, std::io::Error> {
    let id = PasteId::new(3);
    let db_post = models::Post{
        id: 123456,
        username: "".to_string(),
        postdata: paste.data.to_string()
    };
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);
    database::post::create_new_post(db_post, conn);
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: &RawStr) -> Option<File> {
    let file_name = format!("upload/{id}", id = id);
    File::open(&file_name).ok()
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