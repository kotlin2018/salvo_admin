use captcha_rust::Captcha;
use rbatis::crud::CRUD;
use salvo::prelude::*;
use serde::de::Unexpected::Option;
use crate::controller::{encrypt_password, JsonResult};
use crate::dao::{init_rbatis,request_data::SearchReq,request_data::PageParams};
use crate::dao::request_data::UserLoginReq;
use crate::dao::response_data::{AuthBodyResp, CaptchaImageResp};
use crate::entity::user::UserEntity;
use crate::Text::Js;

// #[fn_handler]
// pub async fn get_sort_list(page_params: PageParams,search: Search) ->Json<ListData<UserResp>>{
//   let rb = init_rbatis().await;
//
//   Json(page_params)
// }

/// 获取验证码图片
#[fn_handler]
pub async fn get_captcha() -> Json<JsonResult<CaptchaImageResp>>{
    let captcha = Captcha::new(4,130,40);
    let uuid = encrypt_password(&captcha.text,"");
    let data = CaptchaImageResp{
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    };
    Json(JsonResult{
        code: Some(200),
        msg: Some("success".to_string()),
        data: Some(data),
    })
}

#[fn_handler]
pub async fn login(login_req: UserLoginReq) -> Json<JsonResult<UserEntity>>{//-> Json<&mut JsonResult<AuthBodyResp>>
    let mut msg = "登陆成功".to_string();
    let mut status = "1".to_string();
    if encrypt_password(&login_req.code,"") != login_req.uuid {
        msg = "验证码错误".to_string();
        status = "0".to_string();
    }
    let mut res = JsonResult{
        code: None,
        msg: None,
        data: None
    };
    // 根据用户名获取用户信息
    let rb = init_rbatis().await;
    let w = rb.new_wrapper().eq("user_name", &login_req.user_name);
    let user = rb.fetch_by_wrapper::<UserEntity>(w).await.unwrap();
   //let user = rb.fetch_by_column::<UserEntity,_>("user_name", &login_req.user_name).await.unwrap();
    let user1 = user.clone();
    let user2 = user.clone();
    if &user.user_status.unwrap() == "0" {
        msg = "用户已经被禁用".to_string();
        status = "0".to_string();
        res.code = Some(400);
        res.msg = Some(msg);
        res.data = Some(user1);
        Json(res)
    }else {
        res.code = Some(200);
        res.msg = Some("success".to_string());
        res.data = Some(user2);
        Json(res)
    }
}
#[fn_handler]
pub async fn get_sort_list(page_params: PageParams,search: SearchReq){
  // let rb = init_rbatis().await;
  //   rb.

}

#[fn_handler]
pub async fn get_by_id(req: &mut Request) -> &'static str{
    "get_by_id"
}

#[fn_handler]
pub async fn get_profile(req: &mut Request) -> &'static str{
    "get_profile"
}


#[fn_handler]
pub async fn add_user(req: &mut Request) -> &'static str{
    "add_user"
}

#[fn_handler]
pub async fn delete_user(req: &mut Request) -> &'static str{
    "delete_user"
}

#[fn_handler]
pub async fn edit_user(req: &mut Request) -> &'static str{
    "edit_user"
}

#[fn_handler]
pub async fn update_profile(req: &mut Request) -> &'static str{
    "update_profile"
}

#[fn_handler]
pub async fn get_info(req: &mut Request) -> &'static str{
    "get_info"
}

#[fn_handler]
pub async fn reset_passwd(req: &mut Request) -> &'static str{
    "reset_passwd"
}

#[fn_handler]
pub async fn update_passwd(req: &mut Request) -> &'static str{
    "update_passwd"
}

#[fn_handler]
pub async fn change_status(req: &mut Request) -> &'static str{
    "change_status"
}

#[fn_handler]
pub async fn fresh_token(req: &mut Request) -> &'static str{
    "fresh_token"
}

#[fn_handler]
pub async fn change_role(req: &mut Request) -> &'static str{
    "change_role"
}

#[fn_handler]
pub async fn change_dept(req: &mut Request) -> &'static str{
    "change_dept"
}

#[fn_handler]
pub async fn update_avatar(req: &mut Request) -> &'static str{
    "update_avatar"
}






