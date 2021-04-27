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
mod admin;
mod models;
mod jwt;
//mod logic;

use database::{conn::DbConn};
//embed_migrations!();


use rocket::{Rocket, http::{RawStr, Cookie}, response::Redirect};
use crate::{router::routers::*, admin::{account, menu}};
use rocket_contrib::serve::StaticFiles;

////////////////////////////////////
fn rocket() -> Rocket {

    rocket::ignite().attach(DbConn::fairing())
    .mount("/",routes![index,retrieve,register,login, pos_service, service, new_post, category_list,
    hot_goods, get_goods])
    .mount("/account", routes![account::router::login_admin
                                    ,account::router::info_admin])
    .mount("/menu", routes![menu::router::list_admin])
    .mount("/file", StaticFiles::from("static"))
}

fn main() {
    let rocket = rocket();
    //let conn = PgConn::get_one(&rocket).unwrap();
    //run_migration(&*conn);
    rocket.launch();
}