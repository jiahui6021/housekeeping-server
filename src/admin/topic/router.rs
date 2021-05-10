use super::{models::*, logic};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Form, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo}};

// #[get("/list")]
// pub fn get_topic(conn: DbConn) -> ApiResponse {

// }


// #[post("/list")]
// pub fn get_topic(conn: DbConn) -> ApiResponse {
    
// }

#[post("/article", data = "<article>")]
pub fn post_article(article: Json<JsonArticle>, conn: DbConn) -> ApiResponse {
    let new_article = NewArticle {
        author: article.author.clone(),
        content: article.content.clone(),
        idChannel: article.idChannel.clone(),
        img: article.img.clone(),
        title: article.title.clone()
    };
    if article.id.is_some(){
        logic::update_article(new_article, article.id.unwrap(), &conn);
    } else {
        logic::save_article(new_article, &conn);
    }
    
    ApiResponse {
        json: json!(get_ok_resp("")),
        status: Status::Ok
    }
}

#[get("/article/list?<page>&<limit>&<title>")]
pub fn get_article(page: i32, limit: i32, title: Option<String>, conn: DbConn) -> ApiResponse {
    match logic::get_article_by_page(page, title, limit, &conn){
        Some((articles, sum)) => {
            let resp = ArticleList {
                records: articles,
                current: page,
                limit,
                offset: limit,
                pages: page,
                searchCount: true,
                size: limit,
                total: sum,
            };
            ApiResponse {
                json: json!(get_ok_resp(resp)),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::BadRequest
            }
        }
    }
}

#[get("/article?<id>")]
pub fn get_id_article(id: i32, conn: DbConn) -> ApiResponse {
    match logic::get_article_by_id(id, &conn){
        Some(articles) => {
            ApiResponse {
                json: json!(get_ok_resp(articles)),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::BadRequest
            }
        }
    }
}

#[delete("/article?<id>")]
pub fn del_id_topic(id: i32, conn: DbConn) -> ApiResponse {
    logic::del_article_by_id(id, &conn); 
    ApiResponse {
        json: json!(get_ok_resp("")),
        status: Status::Ok
    }
}

#[post("/topic?<title>&<idArticle>&<idGoodsList>&<pv>&<id>")]
pub fn post_topic(title: String, idArticle: i32, idGoodsList: String,
    pv: String, id: String, conn: DbConn) -> ApiResponse {
    let new_topic = NewTopic {
        id_article: idArticle,
        disabled: false,
        idGoodsList,
        pv: 0,
        title,
    };
    logic::save_topic(new_topic, &conn);
    
    ApiResponse {
        json: json!(get_ok_resp("")),
        status: Status::Ok
    }
}

#[get("/topic/list?<page>&<limit>&<disabled>")]
pub fn get_topic(page: i32, limit: i32, disabled: Option<bool>, conn: DbConn) -> ApiResponse {
    match logic::get_topic_by_page(page, disabled, limit, &conn){
        Some((articles, sum)) => {
            let resp = TopicList {
                records: TopicResp::from_vec(articles, &conn),
                current: page,
                limit,
                offset: limit,
                pages: page,
                searchCount: true,
                size: limit,
                total: sum,
            };
            ApiResponse {
                json: json!(get_ok_resp(resp)),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::BadRequest
            }
        }
    }
}

#[get("/topic/list")]
pub fn get_all_topic(conn: DbConn) -> ApiResponse {
    match logic::get_all_topic(&conn) {
        Some(topics ) => {
            let resp = TopicResp::from_vec(topics, &conn);
            ApiResponse {
                json: json!(get_ok_resp(resp)),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::BadRequest
            }
        }
    }
}

#[get("/topic/<id>")]
pub fn get_id_topic(id: i32, conn: DbConn) -> ApiResponse {
    match logic::get_id_topic(id, &conn) {
        Some(topics ) => {
            let resp = TopicResp::from_topic(topics, &conn);
            ApiResponse {
                json: json!(get_ok_resp(resp)),
                status: Status::Ok
            }
        }
        None => {
            ApiResponse {
                json: json!(get_ok_resp("")),
                status: Status::BadRequest
            }
        }
    }
}
