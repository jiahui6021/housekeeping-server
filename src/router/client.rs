use serde::{Deserialize, Serialize};
pub use crate::database::models::{NewUser};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}


