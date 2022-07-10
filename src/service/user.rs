use std::borrow::Cow;
use anyhow::{Result, Ok, anyhow};
use user_agent_parser::UserAgentParser;
use crate::dto::request_data::UserLoginReq;
use crate::dto::response_data::{AuthBodyResp, UserAgentResp};
use crate::{ApplicationConfig, init_rbatis, Request};

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

/// 用户登陆
pub async fn user_login(login_req: UserLoginReq,req: &mut Request) -> Result<AuthBodyResp>{
    let mut msg = "登陆成功".to_string();
    let mut status = "1".to_string();
    if encrypt_password(&login_req.code,"") != login_req.uuid {
        msg = "验证码错误".to_string();
        status = "0".to_string();
        set_login_info(req,"".to_string(),login_req.user_name.clone(),msg.clone(),status.clone(),None,None).await;
        Err(anyhow!(msg))
    }

    // 根据用户名获取用户信息
    // let rb = init_rbatis().await;
    // let w = rb.new_wrapper().eq("user_name", &login_req.user_name);
    // let user = rb.fetch_by_wrapper::<UserEntity>(w).await.unwrap();
    // //let user = rb.fetch_by_column::<UserEntity,_>("user_name", &login_req.user_name).await.unwrap();
    // let user1 = user.clone();
    // let user2 = user.clone();
    // if &user.user_status.unwrap() == "0" {
    //     msg = "用户已经被禁用".to_string();
    //     status = "0".to_string();
    //     res.code = Some(400);
    //     res.msg = Some(msg);
    //     res.data = Some(user1);
    //     Json(res)
    // }else {
    //     res.code = Some(200);
    //     res.msg = Some("success".to_string());
    //     res.data = Some(user2);
    //     Json(res)
    // }
    Ok()
}

pub async fn set_login_info(
    req: &mut Request,
    uid: String,user: String,msg: String,status: String,
    token_id: Option<String>,token: Option<AuthBodyResp>)
{
    let user_agent: String = req.header("user-agent").unwrap();
    let ua = get_user_agent(&user_agent);
    let ip = get_remote_ip(req);






    // let xff:Option<String> = req.header("X-Forwarded-For");
    // let ip = match xff {
    //     Some(x) => {
    //         let mut ips = x.to_str
    //     }
    // }
}

/// 将从请求头中获取的 user-agent 解析成 UserAgentResp 对象
pub fn get_user_agent(user_agent: &str) -> UserAgentResp {
    let ua_parser = UserAgentParser::
    from_path(&ApplicationConfig::default().user_agent_parser).unwrap();
    let product_v = ua_parser.parse_product(&user_agent);
    let os_v = ua_parser.parse_os(&user_agent);
    let device_v = ua_parser.parse_device(&user_agent);
    let browser = product_v.name.unwrap_or(Cow::Borrowed("")).to_string() + " " + product_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let os = os_v.name.unwrap();//os_v.name.unwrap_or(Cow::Borrowed("")).to_string() + " " + os_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let device = device_v.name.unwrap_or(Cow::Borrowed("")).to_string();
    UserAgentResp{
        browser: browser.trim().to_string(),
        os: os.trim().to_string(),
        device,
    }
}

/// 获取客户端的 IP 地址
pub fn get_remote_ip(req: &mut Request) -> String {
    let ip: String = req.header("X-Forwarded-For").unwrap();
    if ip.is_empty() {
        let x_real_ip: String = req.header("X-Real-IP").unwrap();
        if x_real_ip.is_empty() {
            req.remote_addr().unwrap().to_string()
        }else {
            x_real_ip
        }
    }else{
        ip
    }
}