use log::debug;
use rocket::{Rocket, data, http::{Status, ContentType}, request::Request, response::{self, Responder, Response}};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiJson<T> {
    code: i32,
    msg: String,
    data: T
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn get_ok_resp<T>(data: T) -> ApiJson<T> {
    ApiJson {
        code: 20000,
        msg: "成功".to_string(),
        data
    }
}