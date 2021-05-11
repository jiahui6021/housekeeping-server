use crate::{database::conn::DbConn, schema::{category::{self, dsl}, article, topic}, jwt::JWT};
use super::models;
use diesel::prelude::*;

pub fn save_article(article: models::NewArticle, conn: &DbConn){
    diesel::insert_into(article::table)
            .values(&article)
            .execute(&**conn)
            .expect("Error saving new article");
}

pub fn get_article_by_page(page: i32, title: Option<String>, limit: i32, conn: &DbConn) -> Option<(Vec<models::Article>, i32)> {
    let all_goods = match title {
        Some(title) => {
            article::dsl::article
            .filter(article::dsl::title.eq(title))
            .load::<models::Article>(&**conn)
            .ok()
        }
        None => {
            article::dsl::article
            .load::<models::Article>(&**conn)
            .ok()
        }
    };
    get_limit_article_resp(all_goods, page, limit, conn)
}

fn get_limit_article_resp(all_goods: Option<Vec<models::Article>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::Article>, i32)> {
    let start = (page - 1) * limit;
    let end = start + limit -1;
    let mut resp = Vec::new();
    match all_goods {
        Some(all_goods) => {
            for index in start..end {
                if let Some(good) = all_goods.get(index as usize){
                    resp.push(good.clone());
                }
            }
            Some((resp, all_goods.len() as i32))
        }
        None => {
            None
        }
    }
}

pub fn get_article_by_id(id: i32, conn: &DbConn) -> Option<models::Article> {
    article::dsl::article
    .filter(article::dsl::id.eq(id))
    .first::<models::Article>(&**conn)
    .ok()
}

pub fn del_article_by_id(id: i32, conn: &DbConn) {
    diesel::delete(article::dsl::article.filter(article::dsl::id.eq(id)))
    .execute(&**conn)
    .expect("Error delete article");
}

pub fn update_article(new_article: models::NewArticle, id: i32, conn: &DbConn) {
    let article = models::Article {
        id,
        author: new_article.author,
        content: new_article.content,
        idChannel: new_article.idChannel,
        img: new_article.img,
        title: new_article.title
    };
    diesel::update(article::dsl::article.filter(article::dsl::id.eq(id)))
    .set(article)
    .execute(&**conn)
    .expect("Error update article");
}

pub fn save_topic(topic: models::NewTopic, conn: &DbConn){
    diesel::insert_into(topic::table)
            .values(&topic)
            .execute(&**conn)
            .expect("Error saving new topic");
}

pub fn get_topic_by_page(page: i32, disabled: Option<bool>, limit: i32, conn: &DbConn) -> Option<(Vec<models::Topic>, i32)> {
    let all_goods = match disabled {
        Some(title) => {
            topic::dsl::topic
            .filter(topic::dsl::disabled.eq(title))
            .load::<models::Topic>(&**conn)
            .ok()
        }
        None => {
            topic::dsl::topic
            .load::<models::Topic>(&**conn)
            .ok()
        }
    };
    get_limit_topic_resp(all_goods, page, limit, conn)
}

pub fn get_all_topic(conn: &DbConn) -> Option<Vec<models::Topic>> {
    topic::dsl::topic
    .load::<models::Topic>(&**conn)
    .ok()
}

fn get_limit_topic_resp(all_goods: Option<Vec<models::Topic>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::Topic>, i32)> {
    let start = (page - 1) * limit;
    let end = start + limit -1;
    let mut resp = Vec::new();
    match all_goods {
        Some(all_goods) => {
            for index in start..end {
                if let Some(good) = all_goods.get(index as usize){
                    resp.push(good.clone());
                }
            }
            Some((resp, all_goods.len() as i32))
        }
        None => {
            None
        }
    }
}

pub fn get_id_topic(id: i32, conn: &DbConn) -> Option<models::Topic> {
    topic::dsl::topic
    .filter(topic::dsl::id.eq(id))
    .first::<models::Topic>(&**conn)
    .ok()
}

pub fn del_id_topic(id: i32, conn: &DbConn) {
    diesel::delete(topic::dsl::topic.filter(topic::dsl::id.eq(id)))
    .execute(&**conn)
    .expect("Error delete topic");
}


pub fn change_topic_disabled(id: i32, disabled: bool, conn: &DbConn) {
    diesel::update(topic::dsl::topic.filter(topic::dsl::disabled.eq(disabled)))
    .set(topic::dsl::disabled.eq(disabled))
    .execute(&**conn)
    .expect("Error update article");
}