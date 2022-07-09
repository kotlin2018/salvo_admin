use salvo::Router;
use serde::{Deserialize, Serialize};
use crate::controller::user_controller::{add_user, change_dept, change_role, change_status, delete_user, edit_user, fresh_token, get_by_id, get_captcha, get_info, get_profile, get_sort_list, login, reset_passwd, update_avatar, update_passwd, update_profile};
mod user_controller;
mod role_controller;
mod post_controller;
mod oper_log_controller;
mod menu_controller;
mod login_log_controller;

use salvo::prelude::*;
use tracing::subscriber::with_default;

#[fn_handler]
pub async fn hello() -> &'static str{
    "hello world"
}

#[fn_handler]
pub async fn world() -> &'static str{
    "aaaaaaaaaaaaaaa"
}
// 初始化路由
pub fn init_router() ->Router{
   let  router = Router::new()
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
            .push(Router::with_path("change_role").put(change_role))
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
       .push(Router::with_path("api/system/comm")
           .push(Router::with_path("get_captcha").get(get_captcha))
           .push(Router::with_path("login").post(login)));
   router
}

#[derive(Debug,Serialize,Deserialize)]
pub struct JsonResult<T>{
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

/// 秘密加密
pub fn encrypt_password(password: &str,salt: &str) -> String{
    use std::fmt::Write;
    let s = password.to_owned() + salt;
    let digest = md5::compute(s).to_vec();
    let mut result = String::new();
    for a in digest.iter(){
        write!(result,"{:02x}",a).unwrap();
    }
    result
}
