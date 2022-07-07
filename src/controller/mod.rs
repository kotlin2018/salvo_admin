use salvo::Router;
use serde::{Serialize,Deserialize};
pub mod user_controller;
mod role_controller;
mod post_controller;
mod oper_log_controller;
mod menu_controller;
mod login_log_controller;

// 初始化路由
pub fn init_router() ->Router{
   let router = Router::new();
    router
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Result<T>{
    code: Option<u32>,
    msg: Option<String>,
    data:Option<T>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ListData<T>{
    pub list: Vec<T>,
    pub total: usize,
    pub total_pages: usize,
    pub page_num: usize,
}