use super::{models::Post, conn::DbConn};
use crate::schema::post;
use diesel::prelude::*;
pub fn create_new_post(post: Post, conn: DbConn){
    diesel::insert_into(post::table)
            .values(&post)
            .execute(&*conn)
            .unwrap();
}