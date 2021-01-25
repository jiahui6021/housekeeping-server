#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate diesel;
mod paste_id;
mod database;

use paste_id::PasteId;
use database::conn::PgConn;
//embed_migrations!();

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket::{Data, Rocket, http::RawStr};
use std::{fs::File, path::Path};
#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = PasteId::new(3);
    let file_name = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&file_name))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: &RawStr) -> Option<File> {
    let file_name = format!("upload/{id}", id = id);
    File::open(&file_name).ok()
}

fn rocket() -> Rocket{
    rocket::ignite().attach(PgConn::fairing()).mount("/", routes![index,upload,retrieve])
}

// fn run_migration(db_con: &diesel::PgConnection){
//     embedded_migrations::run_with_output(db_con, &mut std::io::stdout())
//         .expect("migration failed.");
//     log::info!("Database migration finished.");
// }

fn main() {
    let rocket = rocket();
    let conn = PgConn::get_one(&rocket).unwrap();
    //run_migration(&*conn);
    rocket.launch();
}