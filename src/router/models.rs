use serde::{Deserialize, Serialize};
pub use crate::database::models::{NewUser, Users};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct FeedItem {
    pub id: i32,
    pub name: String,
    pub price: String,
    pub tab: i32,
    pub image_key: String,
    pub position: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub feed_item: Vec<FeedItem>,
    pub last_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Discuss {
    pub id: i32,
    pub title: String,
    pub data: String,
    pub like: i32,
    pub service: i32,
    pub position: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub position: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryList {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct GoodsResp {
    pub goods: Goods,
    pub sku: Sku
}

#[derive(Serialize, Deserialize)]
pub struct Goods {
    pub id: i32,
    pub name: String,
    pub descript: String,
    pub pic: String,
    pub price: i32,
    pub isHot: bool,
    pub isNew: bool,
    pub gallery: String,
    pub stock: i32,
    pub isOnSale: bool
}

#[derive(Serialize, Deserialize)]
pub struct Sku {
    pub collection_id: i32,
    pub list: Vec<SkuList>,
    pub none_sku: bool,
    pub price: i32,
    pub stock_num: i32,
    pub tree: Vec<SkuTree>
}

#[derive(Serialize, Deserialize)]
pub struct SkuList {
    pub code: String,
    pub id: i32,
    pub price: i32,
    pub s1: i32,
    pub s2: i32,
    pub stock_num: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SkuTree {
    pub k: String,
    pub k_s: String,
    pub v: Vec<SkuV>
}

#[derive(Serialize, Deserialize)]
pub struct SkuV {
    pub id: i32,
    pub name: String,
    pub plain: bool
}
