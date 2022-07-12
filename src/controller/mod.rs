use salvo::Router;
use serde::{Deserialize, Serialize};
mod role;
mod post;
mod oper;
mod menu;
mod login_log;
mod user;
use crate::controller::user::UserController;
use salvo::prelude::*;
use salvo_extra::cors::CorsHandler;
use tracing::subscriber::with_default;

#[fn_handler]
pub async fn cors(req: &mut Request,res: &mut Response){
    let origin = req.header::<String>("Origin").unwrap();
    println!("origin = {:?}",origin);
}


// 初始化路由
pub fn init_router() ->Router{
    // let cors_handler = CorsHandler::builder()
    //     .with_allow_origin("http://127.0.0.1:3000")
    //     .with_allow_methods(vec!["GET", "POST", "DELETE"])
    //     .build();
   let  router = Router::new().hoop(cors)
        .push(Router::with_path("api/system/user")
            .push(Router::with_path("get_sort_list").get(UserController::get_sort_list))
            .push(Router::with_path("get_by_id").get(UserController::get_by_id))
            .push(Router::with_path("get_profile").get(UserController::get_profile))
            .push(Router::with_path("add").post(UserController::add_user))
            .push(Router::with_path("delete").delete(UserController::delete_user))
            .push(Router::with_path("edit").put(UserController::edit_user))
            .push(Router::with_path("update_profile").put(UserController::update_profile))
            .push(Router::with_path("get_info").get(UserController::get_info))
            .push(Router::with_path("reset_passwd").put(UserController::reset_passwd))
            .push(Router::with_path("update_passwd").put(UserController::update_passwd))
            .push(Router::with_path("change_status").put(UserController::change_status))
            .push(Router::with_path("fresh_token").put(UserController::fresh_token))
            .push(Router::with_path("change_role").put(UserController::change_role))
            .push(Router::with_path("change_dept").put(UserController::change_dept))
            .push(Router::with_path("update_avatar").post(UserController::update_avatar))
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
           .push(Router::with_path("get_captcha").get(UserController::get_captcha))
           .push(Router::with_path("login").post(UserController::login)));
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
