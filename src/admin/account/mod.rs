use crate::database::conn::DbConn;
use models::User;

pub mod models;
pub mod router;

pub fn check_user_admin(id: i32, conn: &DbConn) -> bool {
    let user = User::from_id(id, &conn);
    match user {
        Some(user) => {
            if user.roleid.eq("1") {
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