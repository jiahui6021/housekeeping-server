use crate::{database::conn::DbConn, schema::{category::{self, dsl}, goods, banner, cat_banner, like}, jwt::JWT};
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
                    banner_id: category.banner_id
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

pub fn update_category_banner(id: i32, banner_id: i32, conn: &DbConn) {
    let new_cat_banner = models::NewCarBanner {
        car_id: id,
        banner_id: banner_id,
    };
    diesel::insert_into(cat_banner::table)
            .values(&new_cat_banner)
            .execute(&**conn)
            .expect("Error saving new cat banner");
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

pub fn get_goods_resp_by_page(page: i32, limit: i32, name: Option<String>, conn: &DbConn) -> Option<(Vec<models::GoodsResp>, i32)> {
    let all_goods = match name {
        Some(name) => {
            goods::dsl::goods
            .filter(goods::dsl::name.eq(name))
            .load::<models::Goods>(&**conn)
            .ok()
        }
        None => {
            goods::dsl::goods
            .load::<models::Goods>(&**conn)
            .ok()
        }
    };
    get_limit_goods_resp(all_goods, page, limit, conn)
}

pub fn get_like_goods_by_like(likes: Vec<models::Like>, conn: &DbConn) -> Vec<models::LikeGoods> {
    likes.iter().map(|like|{
        let good = goods::dsl::goods
        .filter(goods::dsl::id.eq(like.id))
        .first::<models::Goods>(&**conn)
        .unwrap();
        models::LikeGoods{
            id: like.id,
            idGoods: good.id,
            goods: models::GoodsResp::from_goods(good, conn),
        }
    }).collect()
}

pub fn get_like_admin_by_like(likes: Vec<models::Like>, conn: &DbConn) -> Vec<models::LikeAdmin> {
    likes.iter().map(|like|{
        let good = goods::dsl::goods
        .filter(goods::dsl::id.eq(like.id))
        .first::<models::Goods>(&**conn)
        .unwrap();
        let user = crate::admin::account::logic::get_shop_user_by_id(like.user_id, conn).unwrap_or_default();
        models::LikeAdmin{
            id: like.id,
            idGoods: like.goods_id,
            goods: models::GoodsResp::from_goods(good, conn),
            idUser: like.user_id,
            user,
        }
    }).collect()
}

pub fn get_category_goods_resp_by_page(page: i32, limit: i32, category: i32, conn: &DbConn) -> Option<(Vec<models::GoodsResp>, i32)> {
    let all_goods = goods::dsl::goods
    .filter(goods::dsl::idCategory.eq(category))
    .filter(goods::dsl::isOnSale.eq(true))
    .load::<models::Goods>(&**conn)
    .ok();
    get_limit_goods_resp(all_goods, page, limit, conn)
}

pub fn get_key_goods_resp_by_page(page: i32, limit: i32, key: String, conn: &DbConn) -> Option<(Vec<models::GoodsResp>, i32)> {
    let all_goods = goods::dsl::goods
    .filter(goods::dsl::name.eq(key))
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

pub fn get_new_goods(conn: &DbConn) -> Option<Vec<models::Goods>> {
    goods::dsl::goods
        .filter(goods::dsl::isNew.eq(true))
        .load::<models::Goods>(&**conn)
        .ok()
}

///////////////////////////////////////// banner
pub fn create_banner(banner: models::NewBanner, conn: &DbConn) {
    diesel::insert_into(banner::table)
            .values(&banner)
            .execute(&**conn)
            .expect("Error saving new banner");
}

pub fn get_banner(conn: &DbConn)-> Option<Vec<models::Banner>> {
    banner::dsl::banner
    .load::<models::Banner>(&**conn)
    .ok()
}

pub fn get_banner_by_id(id: i32, conn: &DbConn) -> models::Banner {
    banner::dsl::banner
    .filter(banner::dsl::id.eq(id))
    .first::<models::Banner>(&**conn)
    .unwrap()
}

pub fn get_banner_by_ids(ids: Vec<i32>, conn: &DbConn) -> Vec<models::Banner> {
    let mut resp = Vec::new();
    for id in ids {
        resp.push(get_banner_by_id(id, conn));
    }
    resp
}

pub fn delete_category_banner(car_id: i32, banner_id: i32, conn: &DbConn) {
    diesel::delete(cat_banner::table)
    .filter(cat_banner::dsl::banner_id.eq(banner_id))
    .filter(cat_banner::dsl::car_id.eq(car_id))
    .execute(&**conn)
    .expect("Error delete car_banner");
}

pub fn get_banner_by_cat(cat_id: i32, conn: &DbConn) -> Vec<models::Banner> {
    let cat_banners = cat_banner::dsl::cat_banner
    .filter(cat_banner::dsl::car_id.eq(cat_id))
    .load::<models::CarBanner>(&**conn)
    .unwrap();
    let mut banners = Vec::new();
    for car_banner in cat_banners {
        let banner = get_banner_by_id(car_banner.banner_id, conn);
        banners.push(banner);
    }
    banners
}

////////////////////////////////////////////// like

pub fn create_like(new_like: models::NewLike, conn: &DbConn) {
    diesel::insert_into(like::table)
            .values(&new_like)
            .execute(&**conn)
            .expect("Error saving new like");
}

pub fn get_like_by_user(user_id: i32, conn: &DbConn) -> Vec<models::Like> {
    like::dsl::like
    .filter(like::dsl::user_id.eq(user_id))
    .load::<models::Like>(&**conn)
    .unwrap()
}

pub fn del_like(user_id: i32, mut goods_id: String, conn: &DbConn) {
    goods_id.remove(goods_id.len()-1);
    goods_id.remove(0);
    let ids = crate::util::split_string_to_i32_vec(goods_id);
    for id in ids {
        diesel::delete(like::table)
        .filter(like::dsl::user_id.eq(user_id))
        .filter(like::dsl::goods_id.eq(id))
        .execute(&**conn)
        .expect("Error delete like");
    }
}

pub fn get_like_num(conn: &DbConn) -> i32 {
    use crate::schema::like::dsl::*;
    use diesel::dsl;
    like.select(dsl::count_star()).first::<i64>(&**conn).unwrap() as i32
}

fn get_limit_like_admin_resp(all_goods: Option<Vec<models::Like>>, page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::Like>, i32)> {
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

pub fn get_like_admin_by_page(page: i32, limit: i32, conn: &DbConn) -> Option<(Vec<models::Like>, i32)> {
    let all_goods = like::dsl::like
    .load::<models::Like>(&**conn)
    .ok();
    get_limit_like_admin_resp(all_goods, page, limit, conn)
}

pub fn if_like(user_id: i32, goods_id:i32, conn: &DbConn) -> bool {
    like::dsl::like
    .filter(like::dsl::user_id.eq(user_id))
    .filter(like::dsl::goods_id.eq(goods_id))
    .load::<models::Like>(&**conn)
    .is_ok()
}

pub fn del_id_like(user_id: i32, goods_id: i32, conn: &DbConn) {
    diesel::delete(like::table)
    .filter(like::dsl::user_id.eq(user_id))
    .filter(like::dsl::goods_id.eq(goods_id))
    .execute(&**conn)
    .expect("Error delete like");
}
