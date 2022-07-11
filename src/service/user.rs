use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Error;
use anyhow::{Result, Ok, anyhow};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use rbatis::crud::CRUD;
use scru128::scru128_string;
use user_agent_parser::UserAgentParser;
use crate::dto::request_data::{AuthPayload, Claims, UserLoginReq};
use crate::dto::response_data::{AuthBodyResp, ClientNetResp, ClientResp, UserAgentResp};
use crate::{ApplicationConfig, init_rbatis, Request};
use crate::entity::user::UserEntity;

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
        return Err(anyhow!(msg))
    }

    // 根据用户名获取用户信息
    let rb = init_rbatis().await;
    let user = rb.fetch_by_column::<UserEntity, _>("user_name", &login_req.user_name).await.unwrap_or_default();
    if user.id.is_none(){
        msg = "用户不存在".to_string();
        status = "0".to_string();
        set_login_info(req, "".to_string(), login_req.user_name.clone(), msg.clone(), status.clone(), None, None).await;
        return Err(anyhow!(msg))
    }else if &user.user_status.unwrap() == "0"{
        msg = "用户已被禁用".to_string();
        status = "0".to_string();
        // set_login_info 传 &login_req.user_name 会报错，只能传 login_req.user_name 对象
        set_login_info(req,"".to_string(),login_req.user_name.clone(),msg.clone(),status.clone(), None, None).await;
        return Err(anyhow!(msg))
    }

    // 验证密码是否正确
    if encrypt_password(&login_req.user_password,&user.user_salt.unwrap()) != user.user_password.unwrap() {
        msg = "密码错误".to_string();
        status = "0".to_string();
        set_login_info(req, "".to_string(), login_req.user_name.clone(), msg.clone(), status.clone(), None, None).await;
        return Err(anyhow!("密码不正确"));
    }

    // 注册 JWT
    let claims = AuthPayload {
        id: user.id.clone().unwrap(),
        name: login_req.user_name.clone()
    };
    let token_id = scru128_string();
    let token = authorize(claims.clone(),token_id.clone()).await.unwrap_or_default();
    // 成功登陆后写入登陆日志
    set_login_info(
        req,
        user.id.unwrap().to_string(),
        login_req.user_name.clone(),
        msg.clone(),
        status.clone(),
        Some(token_id),
        Some(token.clone())
    ).await;
    Ok(token)
}

pub async fn set_login_info(
    req: &mut Request,
    uid: String,user: String,msg: String,status: String,
    token_id: Option<String>,token: Option<AuthBodyResp>)
{
    let user_agent: String = req.header("user-agent").unwrap();
    let ua = get_user_agent(&user_agent);
    let ip = get_remote_ip(req);
    let net = get_city_by_ip(&ip).await.unwrap();
    let client = ClientResp{
        net,
        ua
    };
    // 写入登陆日志
    if status == "1"{
        if let(Some(token_id),Some(token)) = (token_id,token){

        }
    }







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
    let ip: String = req.header("X-Forwarded-For").unwrap_or_default();
    if ip.is_empty() {
        let x_real_ip: String = req.header("X-Real-IP").unwrap_or_default();
        if x_real_ip.is_empty() {
            req.remote_addr().unwrap().to_string()
        }else {
            x_real_ip
        }
    }else{
        ip
    }
}

pub async fn get_city_by_ip(ip: &str) -> Result<ClientNetResp> {
    let url = "http://whois.pconline.com.cn/ipJson.jsp?json=true&ip=".to_string() + ip;
    let resp = reqwest::get(url.as_str()).await?.text_with_charset("utf-8").await.unwrap();
    // from_str 将 json 字符串反序列化成 结构体
    let res = serde_json::from_str::<HashMap<String,String>>(resp.as_str())?;
    let location = format!("{}{}",res["pro"],res["city"]);
    let net_work = res["addr"].split(' ').collect::<Vec<&str>>()[1].to_string();
    Ok(ClientNetResp{
        ip: res["ip"].to_string(),
        location,
        net_work,
    })
}

/// 身份认证
pub async fn authorize(payload: AuthPayload,token_id: String) -> Result<AuthBodyResp>{
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(anyhow!("Missing credentials"))
    }
    let cfg = ApplicationConfig::default();
    let iat = Local::now(); // chrono 时间库中的函数
    let exp = iat + Duration::minutes(cfg.jwt_expire);
    let claims = Claims{
        id: payload.id.to_string(),
        token_id: token_id.clone(),
        name: payload.name,
        exp: exp.timestamp()
    };
    // 这是 jsonwebtoken 中的函数
    let token = encode(&Header::default(),&claims, &EncodingKey::from_secret(cfg.jwt_secret.as_ref())).unwrap_or_default();
    Ok(AuthBodyResp{
        token: token + &token_id,
        token_type: "Bearer".to_string(),
        exp: cfg.jwt_expire,
        exp_in: cfg.jwt_expire
    })
}