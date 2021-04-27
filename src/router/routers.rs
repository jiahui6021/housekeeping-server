use crate::database::{self, conn::DbConn, models};
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use super::models::*;

use crate::paste_id::PasteId;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/newpost", data = "<new_post>")]
pub fn new_post(new_post: Json<models::NewPost>, conn: DbConn) -> Result<String, std::io::Error> {
    let db_post = new_post.into_inner();
    let post = database::post::create_new_post(db_post, conn);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = post.id);
    Ok(url)
}

#[get("/feed/<service>/<last_id>")]
pub fn get_feed_by_service(service: i32, last_id: i32, conn: DbConn) -> ApiResponse {
    let posts = database::post::get_feeds_by_service(service, last_id, conn);
    match posts {
        Some(posts) => {
            ApiResponse {
                json: json!(""),
                status: Status::Ok,
            }
        },
        None => {
            ApiResponse {
                json: json!("您所在的位置地区当前服务还未开通"),
                status: Status::BadRequest
            }
        }
    }
}

#[get("/<id>")]
pub fn retrieve(id: &RawStr, conn: DbConn) -> Json<Post> {
    let id_i32: i32 = FromStr::from_str(id.as_str()).unwrap();
    let post_data = database::post::get_post_by_id(id_i32, conn);
    let ret_data = if post_data.is_some(){
        post_data.unwrap().postdata
    } else {
        "error, id error".to_string()
    };
    Json(Post{data: ret_data})
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<NewUser>, conn: DbConn) -> status::Accepted<String> {
    let new_user = database::user::create_new_user(user.into_inner(), conn);
    status::Accepted(Some(format!("id: '{}'", new_user.id)))
}

// #[post("/login", data = "<login>")]
// pub fn login(mut cookies: Cookies, login: Json<Login>, conn: DbConn) -> Status {
//     if check_login_data(&login.username, &login.password, conn) {
//         cookies.add_private(Cookie::new("user_id", 1.to_string()));
//         Status::Accepted
//     } else {
//         Status::Forbidden
//     }
// }

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
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

#[post("/login", data = "<login>")]
pub fn login(mut cookies: Cookies, login: Json<Login>, conn: DbConn) -> ApiResponse {
    match check_login_data(&login.email, &login.password, conn) {
        Some(user) => {
            cookies.add_private(Cookie::new("user_id", 1.to_string()));
            ApiResponse {
                json: json!(user),
                status: Status::Ok,
            }
        },
        None => {
            ApiResponse {
                json: json!(""),
                status: Status::BadRequest
            }
        }
        
    }
}

fn check_login_data(email: &String, password: &String, conn: DbConn) -> Option<Users> {
    let user = database::user::get_user_by_email(email, conn);
    match user {
        Some(user) => {
            if email == &user.email && password == &user.password {
                Some(user)
            }
            else {
                None
            }
        },
        None => None
    }
}

// #[get("/feeds/<tab>/<id>")]
// pub fn feeds(tab: i32, last_id: i32) -> ApiResponse {

// }

#[get("/service/<province>/<city>/<street>")]
pub fn service(province: i32, city: i32, street: i32, conn: DbConn) -> ApiResponse {
    let services = database::service::get_service_by_pos(province, city, street, conn);
    match services {
        Some(service) => {
            ApiResponse {
                json: json!(service),
                status: Status::Ok,
            }
        },
        None => {
            ApiResponse {
                json: json!("您所在的位置暂未开通"),
                status: Status::BadRequest
            }
        }
    }
}

#[get("/pos/service/<province>/<city>/<street>/<name>/<price>")]
pub fn pos_service(province: i32, city: i32, street: i32, name: String, price:i32, conn: DbConn) {
    let new_service = models::NewService {
        province,
        city,
        street,
        name,
        price,
    };
    database::service::create_new_service(new_service, conn);
}

// #[post("/feed", data = "<feed>")]
// pub fn upload(feed: Json<FeedItem>, conn: DbConn) -> Result<String, std::io::Error> {
//     let id = PasteId::new(3);
//     let db_post = models::NewPost{
//         username: "".to_string(),
//         postdata: paste.data.to_string()
//     };
//     let post = database::post::create_new_post(db_post, conn);
//     let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = post.id);
//     Ok(url)
// }

///////////////////////////// mobile h5 ////////////////////////////////

/// 首页顶部 tab
#[get("/category/list")]
pub fn category_list() -> ApiResponse {
    let mut list = Vec::new();
    list.push(CategoryList{
        id: 1,
        name: "保洁".to_string()
    });
    list.push(CategoryList{
        id: 2,
        name: "家修".to_string()
    });
    list.push(CategoryList{
        id: 3,
        name: "搬家".to_string()
    });

    ApiResponse {
        json: json!(get_ok_resp(list)),
        status: Status::Ok,
    }
}

/// 首页热门推荐
#[get("/goods/searchHot")]
pub fn hot_goods() -> ApiResponse {
    let mut list = Vec::new();
    list.push(Goods{
        id: 1,
        name: "找保姆".to_string(),
        descript: "家庭餐饮制作，卫生清洁，照看老人小孩".to_string(),
        pic: "images/test.jpg".to_string(),
        price: 35723,
        isHot: true,
        isNew: true,
        gallery: "".to_string(),
        stock: 123,
        isOnSale: true,
    });

    list.push(Goods{
        id: 2,
        name: "找月嫂".to_string(),
        descript: "专注母婴健康护理，月嫂高标准筛选，专属客服售后，雇主、阿姨双重保险".to_string(),
        pic: "images/test.jpg".to_string(),
        price: 234833,
        isHot: true,
        isNew: true,
        gallery: "".to_string(),
        stock: 213,
        isOnSale: true,
    });

    ApiResponse {
        json: json!(get_ok_resp(list)),
        status: Status::Ok,
    }
}

/// 商品页商品信息
#[get("/goods/<id>")]
pub fn get_goods(id: i32) -> ApiResponse {
    let goods = Goods{
        id: 1,
        name: "找保姆".to_string(),
        descript: "家庭餐饮制作，卫生清洁，照看老人小孩".to_string(),
        pic: "images/test.jpg".to_string(),
        price: 35723,
        isHot: true,
        isNew: true,
        gallery: "images/test.jpg,images/test.jpg,images/test.jpg".to_string(),
        stock: 234,
        isOnSale: true,
    };

    let sku = Sku{
        collection_id: 1,
        list: vec![],
        none_sku: true,
        price: 23432,
        stock_num: 234,
        tree: vec![],
        
    };

    let resp = GoodsResp {
        goods,
        sku
    };

    ApiResponse {
        json: json!(get_ok_resp(resp)),
        status: Status::Ok,
    }
}



///////////////////////////////////////// share router ///////////////////////////////////////////////////////////////////

// /// 首页热门推荐
// #[get("/file/getImgStream?<idFile>")]
// pub fn get_file(idFile: String) -> ApiResponse {
    

//     ApiResponse {
//         json: json!(get_ok_resp(list)),
//         status: Status::Ok,
//     }
// }
///////////////////////////////////////// admin router ///////////////////////////////////////////////////////////////////

