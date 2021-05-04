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
use crate::{router::routers::*, admin::{account, menu, shop, cart, order}};
use rocket_contrib::serve::StaticFiles;

////////////////////////////////////
fn rocket() -> Rocket {

    rocket::ignite().attach(DbConn::fairing())
    .mount("/",routes![index,retrieve,register,login, pos_service, service, new_post,
     shop::router::get_user_category,shop::router::get_goods_user, upload, shop::router::get_hot,
     shop::router::get_good_info, account::router::loginOrReg, account::router::login_by_pass])
    .mount("/account", routes![account::router::login_admin
                                    ,account::router::info_admin])
    .mount("/menu", routes![menu::router::list_admin])
    .mount("/shop", routes![shop::router::add_category,
                                  shop::router::get_category,
                                  shop::router::delete_category,
                                  shop::router::add_goods,
                                  shop::router::get_goods_admin,
                                  shop::router::change_onsale,
                                  shop::router::get_good])
    .mount("/file", StaticFiles::from("static"))
    .mount("/user", routes![account::router::get_user_info,
                                    cart::router::add_cart,
                                    cart::router::get_cart,
                                    cart::router::add_cart_count,
                                    cart::router::del_cart,
                                    account::router::addr_by_user,
                                    account::router::add_addr,
                                    account::router::addr_by_id,
                                    order::router::prepare])
}

fn main() {
    let rocket = rocket();
    //let conn = PgConn::get_one(&rocket).unwrap();
    //run_migration(&*conn);
    rocket.launch();
}