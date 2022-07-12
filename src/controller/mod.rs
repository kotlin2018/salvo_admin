use salvo::Router;
use serde::{Deserialize, Serialize};
mod role;
mod post;
mod oper;
mod menu;
mod login_log;
mod user;
mod middleware;

use salvo::prelude::*;
use tracing::subscriber::with_default;
use crate::controller::middleware::cors;
use crate::controller::user::{add_user, change_dept, change_status, delete_user, edit_user, fresh_token, get_by_id, get_captcha, get_info, get_profile, get_sort_list, login, reset_passwd, update_avatar, update_passwd, update_profile};

// 初始化路由
pub fn init_router() ->Router{
   let  router = Router::new().hoop(cors)
        .push(Router::with_path("api/system/user")
            .push(Router::with_path("get_sort_list").get(get_sort_list))
            .push(Router::with_path("get_by_id").get(get_by_id))
            .push(Router::with_path("get_profile").get(get_profile))
            .push(Router::with_path("add").post(add_user))
            .push(Router::with_path("delete").delete(delete_user))
            .push(Router::with_path("edit").put(edit_user))
            .push(Router::with_path("update_profile").put(update_profile))
            .push(Router::with_path("get_info").get(get_info))
            .push(Router::with_path("reset_passwd").put(reset_passwd))
            .push(Router::with_path("update_passwd").put(update_passwd))
            .push(Router::with_path("change_status").put(change_status))
            .push(Router::with_path("fresh_token").put(fresh_token))
            .push(Router::with_path("change_role").put(change_dept))
            .push(Router::with_path("change_dept").put(change_dept))
            .push(Router::with_path("update_avatar").post(update_avatar))
        )
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       // .path("")
       .path("")
       .push(Router::with_path("api/comm")
           .push(Router::with_path("get_captcha").get(get_captcha))
           .push(Router::with_path("login").post(login)));
   router
}

#[derive(Debug,Serialize,Deserialize)]
pub struct JsonResult<T>{
    code: Option<u32>,
    data:Option<T>,
    msg: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ListData<T>{
    pub list: Vec<T>,
    pub total: usize,
    pub total_pages: usize,
    pub page_num: usize,
}
