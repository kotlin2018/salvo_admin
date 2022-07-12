use captcha_rust::Captcha;
use salvo::prelude::*;
use crate::controller::JsonResult;
use crate::dto::request_data::UserLoginReq;
use crate::dto::response_data::{AuthBodyResp, CaptchaImageResp};
use crate::dto::request_data::{PageParams, SearchReq};
use crate::entity::user::UserEntity;
use crate::service::user::UserService;

pub struct UserController {}

impl UserController {

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
        let uuid = UserService::encrypt_password(&captcha.text,"");
        let data = CaptchaImageResp{
            captcha_on_off: true,
            img: captcha.base_img,
            uuid,
        };
        Json(JsonResult{
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        })
    }

    #[fn_handler]
    pub async fn login(login_req: UserLoginReq,req: &mut Request) -> Json<JsonResult<AuthBodyResp>>{//
        let res = UserService::user_login(login_req,req).await;
        if res.is_ok(){
            Json(JsonResult{
                code: Some(200),
                msg:  Some("success".to_string()),
                data: Some(res.unwrap())
            })
        }else {
            Json(JsonResult{
                code: Some(400),
                msg:  Some("登陆失败".to_string()),
                data: None
            })
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


}







