use super::models::*;
use log::debug;
use rocket::{Rocket, data, http::{RawStr, Cookie, Cookies, Status, ContentType}, request::Request, response::{self, Redirect, status, Responder, Response}};
use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json, JsonValue};
use crate::{database::conn::DbConn, models::{ApiResponse, get_ok_resp}, admin::account::models::{User, TokenUser, AdminInfo}};

#[get("/listForRouter")]
pub fn list_admin(_token_user: TokenUser, conn: DbConn) -> &'static str {
    "{ \"code\": 20000, \"msg\": \"成功\", \"data\": [ { \"id\": 71, \"parentId\": 0, \"path\": \"/shop\", \"component\": \"layout\", \"name\": \"运营管理\", \"num\": 1, \"hidden\": false, \"meta\": { \"title\": \"shopMgr\", \"icon\": \"shop\" }, \"children\": [ { \"id\": 72, \"parentId\": 71, \"path\": \"/shopUser\", \"component\": \"views/shop/shopUser/index\", \"name\": \"会员管理\", \"num\": 1, \"hidden\": false, \"meta\": { \"title\": \"shopUser\", \"icon\": \"user\" }, \"children\": [] }, { \"id\": 75, \"parentId\": 71, \"path\": \"/category\", \"component\": \"views/shop/category/index\", \"name\": \"服务类别\", \"num\": 2, \"hidden\": false, \"meta\": { \"title\": \"category\", \"icon\": \"category\" }, \"children\": [] }, { \"id\": 77, \"parentId\": 71, \"path\": \"/goodsEdit\", \"component\": \"views/shop/goods/edit\", \"name\": \"商品编辑\", \"num\": 2, \"hidden\": true, \"meta\": { \"title\": \"goodsEdit\", \"icon\": \"goods\" }, \"children\": [] }, { \"id\": 78, \"parentId\": 71, \"path\": \"/shopUserDetail\", \"component\": \"views/shop/shopUser/detail\", \"name\": \"用户详情\", \"num\": 2, \"hidden\": true, \"meta\": { \"title\": \"shopUserDetail\", \"icon\": \"user\" }, \"children\": [] }, { \"id\": 79, \"parentId\": 71, \"path\": \"/orderDetail\", \"component\": \"views/shop/order/detail\", \"name\": \"订单详情\", \"num\": 2, \"hidden\": true, \"meta\": { \"title\": \"orderDetail\", \"icon\": \"order\" }, \"children\": [] }, { \"id\": 73, \"parentId\": 71, \"path\": \"/goods\", \"component\": \"views/shop/goods/index\", \"name\": \"服务管理\", \"num\": 3, \"hidden\": false, \"meta\": { \"title\": \"goods\", \"icon\": \"goods\" }, \"children\": [] }, { \"id\": 74, \"parentId\": 71, \"path\": \"/order\", \"component\": \"views/shop/order/index\", \"name\": \"订单管理\", \"num\": 4, \"hidden\": false, \"meta\": { \"title\": \"order\", \"icon\": \"order\" }, \"children\": [] }, { \"id\": 43, \"parentId\": 71, \"path\": \"/banner\", \"component\": \"views/cms/banner/index\", \"name\": \"banner管理\", \"num\": 6, \"hidden\": false, \"meta\": { \"title\": \"banner\", \"icon\": \"banner\" }, \"children\": [] }, { \"id\": 86, \"parentId\": 71, \"path\": \"/favorite\", \"component\": \"views/shop/favorite/index\", \"name\": \"收藏列表\", \"num\": 6, \"hidden\": false, \"meta\": { \"title\": \"favorite\", \"icon\": \"favorite\" }, \"children\": [] } ] }, { \"id\": 82, \"parentId\": 0, \"path\": \"/promotion\", \"component\": \"layout\", \"name\": \"推广管理\", \"num\": 2, \"hidden\": false, \"meta\": { \"title\": \"promotion\", \"icon\": \"promotion\" }, \"children\": [ { \"id\": 83, \"parentId\": 82, \"path\": \"/topic\", \"component\": \"views/promotion/topic/index\", \"name\": \"专题管理\", \"num\": 1, \"hidden\": false, \"meta\": { \"title\": \"topic\", \"icon\": \"topic\" }, \"children\": [] } ] }, { \"id\": 2, \"parentId\": 0, \"path\": \"/cms\", \"component\": \"layout\", \"name\": \"CMS管理\", \"num\": 3, \"hidden\": false, \"meta\": { \"title\": \"cms\", \"icon\": \"documentation\" }, \"children\": [ { \"id\": 46, \"parentId\": 2, \"path\": \"/cms/articleEdit\", \"component\": \"views/cms/article/edit.vue\", \"name\": \"新建文章\", \"num\": 1, \"hidden\": false, \"meta\": { \"title\": \"editArticle\", \"icon\": \"articleEdit\" }, \"children\": [] }, { \"id\": 42, \"parentId\": 2, \"path\": \"/article\", \"component\": \"views/cms/article/index\", \"name\": \"文章管理\", \"num\": 2, \"hidden\": false, \"meta\": { \"title\": \"article\", \"icon\": \"documentation\" }, \"children\": [] } ] } ], \"success\": true }"
}

