use crate::{database::conn::DbConn, schema::{category::{self, dsl}, article, topic}, jwt::JWT};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;
use super::logic;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Default, Clone)]
#[table_name = "article"]
pub struct Article {
    pub id: i32,
    pub author: String,
    pub content: String,
    pub idChannel: String,
    pub img: String,
    pub title: String
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Default, Clone, FromForm)]
#[table_name = "article"]
pub struct NewArticle {
    pub author: String,
    pub content: String,
    pub idChannel: String,
    pub img: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Default, Clone, FromForm)]
pub struct JsonArticle {
    pub id: Option<i32>,
    pub author: String,
    pub content: String,
    pub idChannel: String,
    pub img: String,
    pub title: String
}


#[derive(Serialize, Deserialize)]
pub struct ArticleList {
    pub records: Vec<Article>,
    pub current: i32,
    pub limit: i32,
    pub offset: i32,
    pub pages: i32,
    pub searchCount: bool,
    pub size: i32,
    pub total: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Default, Clone)]
#[table_name = "topic"]
pub struct Topic {
    pub id: i32,
    pub id_article: i32,
    pub disabled: bool,
    pub idGoodsList: String,
    pub pv: i32,
    pub title: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Default, Clone)]
#[table_name = "topic"]
pub struct NewTopic {
    pub id_article: i32,
    pub disabled: bool,
    pub idGoodsList: String,
    pub pv: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopicList {
    pub records: Vec<TopicResp>,
    pub current: i32,
    pub limit: i32,
    pub offset: i32,
    pub pages: i32,
    pub searchCount: bool,
    pub size: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopicResp {
    pub article: Article,
    pub disabled: bool,
    pub id: i32,
    pub idArticle: i32,
    pub idGoodsList: String,
    pub pv: i32,
    pub title: String,
    pub goodsList: Vec<crate::shop::models::GoodsResp>
}

impl TopicResp {
    pub fn from_vec(topics: Vec<Topic>, conn: &DbConn) -> Vec<Self> {
        let mut resp = Vec::new();
        for topic in topics {
            let article = logic::get_article_by_id(topic.id_article, conn).unwrap();
            let topic_resp = Self {
                article,
                disabled: topic.disabled,
                id: topic.id,
                idArticle: topic.id_article,
                idGoodsList: topic.idGoodsList,
                pv: topic.id,
                title: topic.title,
                goodsList: vec![]
            };
            resp.push(topic_resp);
        }
        resp
    }

    pub fn from_topic(topic: Topic, conn: &DbConn) -> Self {
        let article = logic::get_article_by_id(topic.id_article, conn).unwrap();
        let goods_ids = crate::util::split_string_to_i32_vec(topic.idGoodsList.clone());
        let mut goodsList = Vec::new();
        for id in goods_ids {
            let good = crate::admin::shop::logic::get_goods_by_id(id, conn).unwrap();
            goodsList.push(crate::admin::shop::models::GoodsResp::from_goods(good, conn));
        }
        Self {
            article,
            disabled: topic.disabled,
            id: topic.id,
            idArticle: topic.id_article,
            idGoodsList: topic.idGoodsList,
            pv: topic.id,
            title: topic.title,
            goodsList
        }
    }
}

