use serde::{Serialize,Deserialize};
use salvo::macros::Extractible;
use tokio::fs::OpenOptions;

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
    pub user_password: String,
    pub code: String,
    pub uuid: String,
}