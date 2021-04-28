use crate::{database::conn::DbConn, schema::category::{self, dsl}, jwt::JWT};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use rocket::{Request, request::{self, FromRequest}, Outcome, http::Status};
use std::error::Error;

#[derive(Serialize, Deserialize, Default)]
pub struct CategoryResp {
    pub children: Vec<CategoryResp>,
    pub bannerList: Vec<Banner>,
    pub descript: String,
    pub icon: String,
    pub id: i32,
    pub isDelete: bool,
    pub label: String,
    pub name: String,
    pub pid: Option<i32>,
    pub showIndex: bool,
    pub sort: i32,
    pub url: String,
}

impl CategoryResp {
    pub fn from_muti(categorys: Vec<Category>, conn: &DbConn) -> Vec<Self> {
        categorys.into_iter().map(|category|{
            let id = category.id;
            let banners = vec![];
            CategoryResp {
                children: vec![],
                bannerList: banners,
                descript: category.descript,
                icon: category.icon,
                id: category.id,
                isDelete: category.isDelete,
                label: category.label,
                name: category.name,
                pid: category.pid,
                showIndex: category.showIndex,
                sort: category.sort,
                url: category.url,
            }
        }).collect()
        
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Banner {
    pub id: i32,
    pub idFile: String,
    pub page: String,
    pub param: String,
    pub title: String,
    pub r#type: String,
    pub url: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Default)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub descript: String,
    pub icon: String,
    pub url: String,
    pub label: String,
    pub name: String,
    pub showIndex: bool,
    pub isDelete: bool,
    pub sort: i32,
    pub pid: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Default, Clone)]
#[table_name = "category"]
pub struct NewCategory {
    pub descript: String,
    pub icon: String,
    pub url: String,
    pub label: String,
    pub name: String,
    pub showIndex: bool,
    pub isDelete: bool,
    pub sort: i32,
    pub pid: Option<i32>,
}
