use crate::{database::conn::DbConn, schema::category::{self, dsl}, jwt::JWT};
use super::models;
use diesel::prelude::*;
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

/// return false means need create new category
pub fn update_category_by_id(id: Option<i32>, category: models::NewCategory, conn: &DbConn) -> bool {
    match id {
        Some(id) => {
            let db_category = dsl::category
            .filter(dsl::id.eq(id))
            .first::<models::Category>(&**conn)
            .ok();
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
