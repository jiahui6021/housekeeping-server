use crate::{database::conn::DbConn, schema::{category::{self, dsl}, goods, banner, cat_banner, like}, jwt::JWT};
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
            let banners = super::logic::get_banner_by_cat(category.id, conn);
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

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Default, Clone)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub descript: String,
    pub icon: String,
    pub url: String,
    pub label: String,
    pub name: String,
    pub banner_id: String,
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
    pub banner_id: String,
    pub showIndex: bool,
    pub isDelete: bool,
    pub sort: i32,
    pub pid: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsList {
    pub records: Vec<GoodsResp>,
    pub current: i32,
    pub limit: i32,
    pub offset: i32,
    pub pages: i32,
    pub searchCount: bool,
    pub size: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsAdminInfo {
    pub goods: GoodsResp,
    pub skuList: Vec<Sku>
}

#[derive(Serialize, Deserialize)]
pub struct GoodsInfo {
    pub goods: GoodsResp,
    pub sku: Sku
}

impl GoodsInfo {
    pub fn from_goods(goods: Goods, conn: &DbConn) -> Self {
        let sku = Sku {
            collection_id: goods.id,
            list: vec![],
            none_sku: true,
            price: goods.price,
            stock_num: goods.stock,
            tree: vec![]
        };
        Self {
            goods: GoodsResp::from_goods(goods, &conn),
            sku
        }
    }
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "goods"]
pub struct Goods {
    pub id: i32,
    pub name: String,
    pub descript: String,
    pub gallery: String,
    pub pic: String,
    pub detail: String,
    pub price: i32,
    pub stock: i32,
    pub idCategory: i32,
    pub isOnSale: bool,
    pub isHot: bool,
    pub isNew: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GoodsResp {
    pub id: i32,
    pub name: String,
    pub descript: String,
    pub gallery: String,
    pub pic: String,
    pub detail: String,
    pub price: i32,
    pub stock: i32,
    pub idCategory: i32,
    pub isOnSale: bool,
    pub isHot: bool,
    pub isNew: bool,
    pub category: Category
}

impl GoodsResp {
    pub fn from_goods(goods: Goods, conn: &DbConn) -> Self {
        let category = super::logic::get_category_by_id(goods.idCategory, conn).unwrap_or_default();
        Self {
            id: goods.id,
            name: goods.name,
            descript: goods.descript,
            gallery: goods.gallery,
            pic: goods.pic,
            detail: goods.detail,
            price: goods.price,
            stock: goods.stock,
            idCategory: goods.idCategory,
            isOnSale: goods.isOnSale,
            isHot: goods.isHot,
            isNew: goods.isNew,
            category
        }
    }
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, FromForm, Default, Clone)]
#[table_name = "goods"]
pub struct NewGoods {
    pub name: String,
    pub descript: String,
    pub gallery: String,
    pub pic: String,
    pub detail: String,
    pub price: i32,
    pub stock: i32,
    pub idCategory: i32,
    pub isOnSale: bool,
    pub isHot: bool,
    pub isNew: bool,
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

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "banner"]
pub struct Banner {
    pub id: i32,
    pub idFile: String,
    pub page: String,
    pub param: String,
    pub title: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "banner"]
pub struct NewBanner {
    pub idFile: String,
    pub page: String,
    pub param: String,
    pub title: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "cat_banner"]
pub struct CarBanner {
    pub id: i32,
    pub car_id: i32,
    pub banner_id: i32,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "cat_banner"]
pub struct NewCarBanner {
    pub car_id: i32,
    pub banner_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "like"]
pub struct Like {
    pub id: i32,
    pub user_id: i32,
    pub goods_id: i32,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Clone, Default)]
#[table_name = "like"]
pub struct NewLike {
    pub user_id: i32,
    pub goods_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LikeGoods {
    pub id: i32,
    pub idGoods: i32,
    pub goods: GoodsResp
}

#[derive(Serialize, Deserialize)]
pub struct LikeAdmin {
    pub id: i32,
    pub idGoods: i32,
    pub goods: GoodsResp,
    pub idUser: i32,
    pub user: crate::admin::account::models::ShopUser
}

#[derive(Serialize, Deserialize)]
pub struct LikeAdminList {
    pub records: Vec<LikeAdmin>,
    pub current: i32,
    pub limit: i32,
    pub offset: i32,
    pub pages: i32,
    pub searchCount: bool,
    pub size: i32,
    pub total: i32,
}
