use serde::{Deserialize, Serialize};
pub use crate::database::models::{NewUser};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub data: String,
}


