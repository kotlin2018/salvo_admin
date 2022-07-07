use salvo::Router;
use serde::{Deserialize, Serialize};
use crate::controller::user_controller::{get_by_id, get_sort_list};

pub mod user_controller;
mod role_controller;
mod post_controller;
mod oper_log_controller;
mod menu_controller;
mod login_log_controller;
mod request_data;
pub mod response_data;

// 初始化路由
pub fn init_router() ->Router{
   let  router = Router::new()
        .path("/system/user/get_sort_list").get(get_sort_list)
        .push(Router::with_path("/system/user/get_by_id").get(get_by_id))
        .push(Router::with_path("/system/user/get_profile").get(get_by_id));
    return router
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
