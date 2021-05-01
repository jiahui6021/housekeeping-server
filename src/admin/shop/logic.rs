use crate::{database::conn::DbConn, schema::{category::{self, dsl}, goods}, jwt::JWT};
use super::models;
use diesel::prelude::*;
///////////////////// category /////////////////////
pub fn create_category(category: models::NewCategory, conn: &DbConn) {
    diesel::insert_into(category::table)
            .values(&category)
            .execute(&**conn)
            .expect("Error saving new category");
}

pub fn get_all_category(conn: &DbConn)-> Option<Vec<models::Category>> {
    dsl::category
    .load::<models::Category>(&**conn)
    .ok()
}

pub fn get_category_by_id(id: i32, conn: &DbConn) -> Option<models::Category> {
    dsl::category
        .filter(dsl::id.eq(id))
        .first::<models::Category>(&**conn)
        .ok()
}

/// return false means need create new category
pub fn update_category_by_id(id: Option<i32>, category: models::NewCategory, conn: &DbConn) -> bool {
    match id {
        Some(id) => {
            let db_category = get_category_by_id(id, conn);
            if db_category.is_some() {
                let new_category = models::Category{
                    id,
                    descript: category.descript,
                    icon: category.icon,
                    url: category.url,
                    label: category.label,
                    name: category.name,
                    showIndex: category.showIndex,
                    isDelete: category.isDelete,
                    sort: category.sort,
                    pid: category.pid,
                    
                };
                update_category(new_category, conn);
                true
            } else {
                false
            }
        }
        None => {
            false
        }
    }
    
}

pub fn update_category(new_category: models::Category, conn: &DbConn) {
    diesel::update(dsl::category.filter(dsl::id.eq(new_category.id)))
    .set(new_category)
    .execute(&**conn)
    .expect("Error update category");
}

pub fn delete_category_by_id(id: i32, conn: &DbConn) {
    diesel::delete(dsl::category.filter(dsl::id.eq(id)))
    .execute(&**conn)
    .expect("Error delete category");
}

//////////////////////// goods ///////////////////////////////////////

pub fn create_goods(goods: models::NewGoods, conn: &DbConn) {
    diesel::insert_into(goods::table)
            .values(&goods)
            .execute(&**conn)
            .expect("Error saving new category");
}

fn get_limit_goods_resp(all_goods: Option<Vec<models::Goods>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::GoodsResp>, i32)> {
    let start = (page - 1) * limit;
    let end = start + limit -1;
    let mut resp = Vec::new();
    match all_goods {
        Some(all_goods) => {
            for index in start..end {
                if let Some(good) = all_goods.get(index as usize){
                    resp.push(models::GoodsResp::from_goods(good.clone(),conn));
                }
            }
            Some((resp, all_goods.len() as i32))
        }
        None => {
            None
        }
    }
}

pub fn get_goods_resp_by_page(page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::GoodsResp>, i32)> {
    let all_goods = goods::dsl::goods
    .load::<models::Goods>(&**conn)
    .ok();
    get_limit_goods_resp(all_goods, page, limit, conn)
}

pub fn get_category_goods_resp_by_page(page: i32, limit: i32, category: i32, conn: &DbConn) -> Option<(Vec<models::GoodsResp>, i32)> {
    let all_goods = goods::dsl::goods
    .filter(goods::dsl::idCategory.eq(category))
    .filter(goods::dsl::isOnSale.eq(true))
    .load::<models::Goods>(&**conn)
    .ok();
    get_limit_goods_resp(all_goods, page, limit, conn)
}

pub fn change_goods_onsale(id: i32, is_onsale: bool, conn: &DbConn) {
    let mut goods = get_goods_by_id(id, conn).unwrap_or_default();
    goods.isOnSale = is_onsale;
    update_goods(goods, conn);
}

pub fn get_goods_by_id(id: i32, conn: &DbConn) -> Option<models::Goods> {
    goods::dsl::goods
        .filter(goods::dsl::id.eq(id))
        .first::<models::Goods>(&**conn)
        .ok()
}

pub fn update_goods(goods: models::Goods, conn: &DbConn) {
    diesel::update(goods::dsl::goods.filter(goods::dsl::id.eq(goods.id)))
    .set(goods)
    .execute(&**conn)
    .expect("Error update goods");
}

/// return false means need create new goods
pub fn update_goods_by_id(id: Option<i32>, goods: models::NewGoods, conn: &DbConn) -> bool {
    match id {
        Some(id) => {
            let db_goods = get_goods_by_id(id, conn);
            if db_goods.is_some() {
                let new_goods = models::Goods{
                    id: id,
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
                };
                update_goods(new_goods, conn);
                true
            } else {
                false
            }
        }
        None => {
            false
        }
    }
    
}

pub fn get_hot_goods(conn: &DbConn) -> Option<Vec<models::Goods>> {
    goods::dsl::goods
        .filter(goods::dsl::isHot.eq(true))
        .load::<models::Goods>(&**conn)
        .ok()
}