use serde::{Serialize,Deserialize};
use salvo::macros::Extractible;


/// 分页参数
#[derive(Debug,Serialize,Deserialize,Extractible)]
#[extract(default_source(from = "query"))]
pub struct PageParams {
    // #[extract(source(from = "query"))]
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
    pub sort: Option<String>,
}

#[derive(Debug,Serialize,Deserialize,Extractible)]
#[extract(default_source(from = "query"))]
pub struct SearchReq {
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub user_ids: Option<Vec<String>>,
    pub user_name: Option<String>,
    pub phone_num: Option<String>,
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    pub dept_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

/// 用户登陆
#[derive(Debug,Serialize,Deserialize,Extractible)]
#[extract(default_source(from = "body",format = "json"))]
pub struct UserLoginReq {
    pub user_name: String,
    pub password: String,
    pub code: String,
    pub uuid: String,
}

#[derive(Debug,Deserialize,Clone)]
pub struct AuthPayload {
    pub id: String,
    pub name: String,
}

/// 包装了 token 信息的结构体
#[derive(Debug,Serialize,Deserialize,Extractible,Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct Claims {
    pub id: String, //用户id
    pub token_id: String,
    pub name: String, //用户名
    pub exp: i64
}

/// 用户注册信息
#[derive(Debug,Serialize,Deserialize,Extractible,Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct SignUpReq {
    pub phone: String,
    pub password: String,
    pub user_name: String,
    pub nickname: String,
    pub email: String,
    pub sex: String,
    pub avatar: String,
}

/// 新增用户
#[derive(Debug,Serialize,Deserialize,Extractible)]
#[extract(default_source(from = "body", format = "json"))]
pub struct AddUserReq{
    pub user_name: String,
    pub nickname: String,
    pub password: String,
    pub status: String,
    pub email: Option<String>,
    pub gender: String,
    pub avatar: Option<String>,
    pub remark: Option<String>,
    pub is_admin: bool,
    pub phone: String,
    pub post_ids: Vec<String>,
    pub dept_ids: Vec<String>,
    pub dept_id: String,
    pub role_ids: Vec<String>,
    pub role_id: String,
}