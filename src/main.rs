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
mod util;
//mod logic;

use database::{conn::DbConn};
//embed_migrations!();


use rocket::{Rocket, http::{RawStr, Cookie}, response::Redirect};
use crate::{router::routers::*, admin::{account, menu, shop, cart, order, topic}};
use rocket_contrib::serve::StaticFiles;

////////////////////////////////////
fn rocket() -> Rocket {

    rocket::ignite().attach(DbConn::fairing())
    .mount("/",routes![index,
                            retrieve,
                            register,
                            login, 
                            pos_service, 
                            service, 
                            new_post,
                            shop::router::get_user_category,
                            shop::router::get_goods_user, 
                            upload, 
                            shop::router::get_hot,
                            shop::router::get_good_info, 
                            account::router::loginOrReg, 
                            account::router::login_by_pass,
                            order::router::pay_order, 
                            topic::router::post_article, 
                            topic::router::get_article, 
                            topic::router::get_id_article,
                            topic::router::del_id_article, 
                            topic::router::get_all_topic, 
                            topic::router::get_id_topic, 
                            shop::router::add_banner,
                            shop::router::get_banner, 
                            shop::router::get_new, 
                            shop::router::get_goods_key, 
                            account::router::logout_user,
                            account::router::dashboard])
    .mount("/account", routes![account::router::login_admin
                                    ,account::router::info_admin,
                                    account::router::logout,
                                    account::router::update_admin_pass])
    .mount("/menu", routes![menu::router::list_admin])
    .mount("/shop", routes![shop::router::add_category,
                                  shop::router::get_category,
                                  shop::router::delete_category,
                                  shop::router::add_goods,
                                  shop::router::get_goods_admin,
                                  shop::router::change_onsale,
                                  shop::router::get_good,
                                  order::router::get_order_admin,
                                  account::router::get_shop_user_admin,
                                  account::router::get_id_user_info,
                                  account::router::get_user_addr_admin,
                                  shop::router::set_banner,
                                  shop::router::get_id_banner,
                                  shop::router::remove_banner,
                                  order::router::send_out,
                                  order::router::order_msg,
                                  shop::router::get_favorite_admin])
    .mount("/file", StaticFiles::from("static"))
    .mount("/user", routes![account::router::get_user_info,
                                    cart::router::add_cart,
                                    cart::router::get_cart,
                                    cart::router::add_cart_count,
                                    cart::router::del_cart,
                                    account::router::addr_by_user,
                                    account::router::add_addr,
                                    account::router::addr_by_id,
                                    order::router::prepare,
                                    order::router::save_order,
                                    order::router::get_order,
                                    order::router::get_id_order,
                                    shop::router::add_favorite,
                                    shop::router::get_favorite,
                                    shop::router::del_like,
                                    order::router::confirm,
                                    order::router::cancel,
                                    account::router::update_user_name,
                                    account::router::update_user_sex,
                                    account::router::update_user_pass,
                                    topic::router::change_topic_able,
                                    shop::router::if_like,
                                    shop::router::del_favorite])
    .mount("/promotion", routes![topic::router::post_topic, topic::router::get_topic, topic::router::del_id_topic,])
}

fn main() {
    let rocket = rocket();
    //let conn = PgConn::get_one(&rocket).unwrap();
    //run_migration(&*conn);
    rocket.launch();
}