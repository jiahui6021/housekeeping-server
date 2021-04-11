use super::{models, conn::DbConn};
use crate::{schema::users};
use diesel::prelude::*;
pub fn create_new_user(user: models::NewUser, conn: DbConn) -> models::Users {
    diesel::insert_into(users::table)
            .values(&user)
            .execute(&*conn)
            .expect("Error saving new user");
    users::table.order(users::id.desc()).first(&*conn).unwrap()
}

pub fn get_user_by_id(get_id: i32, conn: DbConn) -> Option<models::Users> {
    users::table.find(get_id).first(&*conn).ok()
}