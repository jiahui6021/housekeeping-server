use serde::{Deserialize, Serialize};
pub use crate::database::models::{NewUser, Users};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}


