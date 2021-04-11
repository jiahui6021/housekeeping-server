use super::{models, conn::DbConn};
use crate::{schema::post};
use diesel::prelude::*;
pub fn create_new_post(post: models::NewPost, conn: DbConn) -> models::Post {
    diesel::insert_into(post::table)
            .values(&post)
            .execute(&*conn)
            .expect("Error saving new post");
    post::table.order(post::id.desc())
    .first(&*conn).unwrap()
}

pub fn get_post_by_id(get_id: i32, conn: DbConn) -> Option<models::Post> {
    post::table.find(get_id).first(&*conn).ok()
}