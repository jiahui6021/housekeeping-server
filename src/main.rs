#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;
#[macro_use]
extern crate serde_json;
#[macro_use] 
extern crate rocket_contrib;

mod paste_id;
mod database;
mod schema;
mod router;

use database::{conn::DbConn, models};
//embed_migrations!();


use rocket::{Rocket, http::{RawStr, Cookie}, response::Redirect};
use crate::router::routers::*;

////////////////////////////////////
fn rocket() -> Rocket {
    rocket::ignite().attach(DbConn::fairing())
    .mount("/",routes![index,upload,retrieve,register,login, pos_service, service])
}

fn main() {
    let rocket = rocket();
    //let conn = PgConn::get_one(&rocket).unwrap();
    //run_migration(&*conn);
    rocket.launch();
}