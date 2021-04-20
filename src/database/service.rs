use super::{models, conn::DbConn};
use crate::{schema::service::{self, dsl}};
use diesel::prelude::*;
pub fn create_new_service(service: models::NewService, conn: DbConn) -> models::Service {
    diesel::insert_into(service::table)
            .values(&service)
            .execute(&*conn)
            .expect("Error saving new service");
    service::table.order(service::id.desc())
    .first(&*conn).unwrap()
}

pub fn get_service_by_pos(province: i32, city: i32, street: i32, conn: DbConn) -> Option<Vec<models::Service>> {
    dsl::service
    .filter(dsl::province.eq(province))
    .filter(dsl::city.eq(city))
    .filter(dsl::street.eq(street))
    .load::<models::Service>(&*conn)
    .ok()
    //post::table.find(get_id).first(&*conn).ok()
}