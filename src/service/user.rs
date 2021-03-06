use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::format;
// use anyhow::{Result, Ok, anyhow};
use std::result::Result::{Ok as stdOK,Err as stdErr};
use chrono::{Duration, Local};
use fast_log::init;
use jsonwebtoken::{encode, EncodingKey, Header};
use rbatis::crud::{CRUD, CRUDMut};
use rbatis::{DateTimeNative,Uuid};
use rbson::Bson;
use rbatis::executor::{ExecutorMut, RBatisTxExecutor};
use scru128::scru128_string;
use user_agent_parser::UserAgentParser;
use crate::dto::request_data::{AddUserReq, AuthPayload, Claims, SignUpReq, UserLoginReq};
use crate::dto::response_data::{AuthBodyResp, ClientNetResp, ClientResp, UserAgentResp, UserAndDeptResp};
use crate::{ApplicationConfig, init_rbatis, Request};
use crate::entity::dept::DeptEntity;
use crate::entity::login_log::LoginLogEntity;
use crate::entity::user::UserEntity;
use crate::entity::user_dept::UserDeptEntity;
use crate::entity::user_online::UserOnlineEntity;
use crate::helper::rand_util::rand_string;

pub struct UserService{}

impl UserService {

    /// 秘密加密
    pub fn encrypt_password(password: &str,salt: &str) ->String{
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
    pub async fn user_login(login_req: UserLoginReq,req: &mut Request) -> Result<AuthBodyResp,String>{
        let mut msg = "登陆成功".to_string();
        let mut status = "1".to_string();
        if UserService::encrypt_password(&login_req.code,"") != login_req.uuid {
            msg = "验证码错误".to_string();
            status = "0".to_string();
            UserService::set_login_info(req,"".to_string(),login_req.user_name.clone(),msg.clone(),status.clone(),None,None).await;
            return Err(msg)
        }

        // 根据用户名获取用户信息
        let rb = init_rbatis().await;
        let user = rb.fetch_by_column::<UserEntity, _>("user_name", &login_req.user_name).await.unwrap_or_default();
        if user.id.is_none(){
            msg = "用户不存在".to_string();
            status = "0".to_string();
            UserService::set_login_info(req, "".to_string(), login_req.user_name.clone(), msg.clone(), status.clone(), None, None).await;
            return Err(msg)
        }else if &user.user_status.unwrap() == "0"{
            msg = "用户已被禁用".to_string();
            status = "0".to_string();
            // set_login_info 传 &login_req.user_name 会报错，只能传 login_req.user_name 对象
            UserService::set_login_info(req,"".to_string(),login_req.user_name.clone(),msg.clone(),status.clone(), None, None).await;
            return Err(msg)
        }

        // 验证密码是否正确
        if UserService::encrypt_password(&login_req.password,&user.user_salt.unwrap()) != user.user_password.unwrap() {
            msg = "密码错误".to_string();
            status = "0".to_string();
            UserService::set_login_info(req, "".to_string(), login_req.user_name.clone(), msg.clone(), status.clone(), None, None).await;
            return Err(msg);
        }

        // 注册 JWT
        let claims = AuthPayload {
            id: user.id.clone().unwrap(),
            name: login_req.user_name.clone()
        };
        let token_id = scru128_string();
        let token = UserService::authorize(claims.clone(),token_id.clone()).await.unwrap_or_default();
        // 成功登陆后写入登陆日志
        UserService::set_login_info(
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
        let ua = UserService::get_user_agent(&user_agent);
        let ip = UserService::get_remote_ip(req);
        let net = UserService::get_city_by_ip(&ip).await.unwrap();
        let client = ClientResp{
            net,
            ua
        };
        // 写入登陆日志
        if status == "1"{
            if let(Some(token_id),Some(token)) = (token_id,token){
                UserService::user_online_add(client.clone(),uid,token_id,token.clone().exp).await;
            }
        };
        tokio::spawn(async move {
            UserService::login_log_add(client.clone(),user,msg,status.clone()).await;
        });
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

    pub async fn get_city_by_ip(ip: &str) -> Result<ClientNetResp,String> {
        let url = "http://whois.pconline.com.cn/ipJson.jsp?json=true&ip=".to_string() + ip;
        let resp = reqwest::get(url.as_str()).await.expect("请求失败").text_with_charset("utf-8").await.expect("设置utf-8失败");
        // from_str 将 json 字符串反序列化成 结构体
        let res = serde_json::from_str::<HashMap<String,String>>(resp.as_str()).expect("解析json失败");
        let location = format!("{}{}",res["pro"],res["city"]);
        let net_work = res["addr"].split(' ').collect::<Vec<&str>>()[1].to_string();
        Ok(ClientNetResp{
            ip: res["ip"].to_string(),
            location,
            net_work,
        })
    }

    /// 身份认证
    pub async fn authorize(payload: AuthPayload,token_id: String) -> Result<AuthBodyResp,String>{
        if payload.id.is_empty() || payload.name.is_empty() {
            return Err("Missing credentials".to_string())
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
        match encode(&Header::default(),&claims, &EncodingKey::from_secret(cfg.jwt_secret.as_ref())){
            Ok(token) => Ok(AuthBodyResp{
                token: token + &token_id,
                token_type: "Bearer".to_string(),
                exp: cfg.clone().jwt_expire,
                exp_in: cfg.clone().jwt_expire
            }),
            Err(e) => Err("生成token错误".to_string())
        }
    }

    // 使用 Rbatis 的普通事务
    pub async fn user_online_add(req: ClientResp,uid: String,token_id: String,token_exp: i64) {
        let u_id = scru128::scru128().to_string();
        let now = DateTimeNative::now();
        let rb = init_rbatis().await;

        // 使用 html_sql 属性宏
        let user = UserEntity::get_by_id(&rb,&uid).await.expect("获取用户信息失败");

        // 使用 Wrapper 包装器
        let w = rb.new_wrapper().eq("deleted_at","null").eq("dept_id",&user.dept_id.unwrap());
        let dept = rb.fetch_by_wrapper::<DeptEntity>(w).await.expect("获取部门信息失败");

        // 使用普通事务
        let mut tx = rb.acquire_begin().await.expect("初始化事务句柄失败");
        let user_online = UserOnlineEntity {
            id: Some(u_id.clone()),
            u_id: Some(uid),
            token_id: Some(token_id),
            token_exp: Some(token_exp),
            login_time: Some(now),
            user_name: user.user_name.clone(),
            dept_name: dept.dept_name.clone(),
            net: Some(req.net.net_work),
            ipaddr: Some(req.net.ip),
            login_location: Some(req.net.location),
            device: Some(req.ua.device),
            browser: Some(req.ua.browser),
            os: Some(req.ua.os)
        };
        tx.save(&user_online,&[]).await.expect("初始化事务句柄失败");
        tx.commit().await.expect("提交事务失败");
    }

    pub async fn login_log_add(req: ClientResp,user: String,msg: String,status: String){
        let uid = scru128::scru128().to_string();
        let now = DateTimeNative::now();
        let login_log = LoginLogEntity{
            info_id: Some(uid.clone()),
            login_name: Some(user),
            net: Some(req.net.net_work),
            ipaddr: Some(req.net.ip),
            login_location: Some(req.net.location),
            browser: Some(req.ua.browser),
            os: Some(req.ua.os),
            device: Some(req.ua.device),
            status: Some(status),
            msg: Some(msg),
            login_time: Some(now),
            module: Some("后台系统".to_string())
        };
        let rb = init_rbatis().await;
        let mut tx: RBatisTxExecutor = rb.acquire_begin().await.expect("初始化事务句柄失败");
        tx.save(&login_log,&[]).await.expect("");
        tx.commit().await.expect("提交事务失败");
    }
    
    // 刷新 token
    pub async fn fresh_token(user: Claims) -> Result<AuthBodyResp,String>{
        let claims = AuthPayload {
            id: user.clone().id,
            name: user.clone().name,
        };
        /// 重新生成一个 token
        match UserService::authorize(claims.clone(),user.clone().token_id).await{
            Ok(token) => {
                UserService::update_online(user.clone().token_id,token.clone().exp);
                Ok(token)
            },
            Err(e)=>Err("刷新token失败".to_string())
        }
    }

    // 使用原生事务
    pub async fn update_online(token: String,token_exp: i64) -> Result<String,()> {
        let rb = init_rbatis().await;
        let mut tx = rb.acquire_begin().await.unwrap();
        tx.exec("update sys_user_online set token_exp=? where token_id =?",vec![Bson::Int64(token_exp),Bson::String(token)]).await.expect("");
        tx.commit().await.expect("更新失败");
        Ok("token更新成功".to_string())
    }

    /// 注册用户
    pub async fn sign_up(arg: SignUpReq) -> String{
        let rb = init_rbatis().await;
        let mut tx = rb.acquire_begin().await.expect("事务初始化失败");
        let id = tx.fetch::<u32>("select id from sys_user where phone_num = ?",vec![Bson::String(arg.clone().phone)]).await;
        let mut result = "".to_string();
        if id.is_ok(){
            result = String::from(format!("{} 该手机号已经注册，请直接登陆您的账号",&arg.phone))
        }else {
            let user = UserEntity{
                id: Some(Uuid::new().to_string()),
                user_name: Some(arg.clone().user_name),
                user_nickname: Some(arg.clone().nickname),
                user_password: Some(arg.clone().password),
                user_salt: Some("0000".to_string()),// 这里应该使用随机数
                user_status: Some("已注册".to_string()),
                user_email: Some(arg.clone().email),
                sex: Some(arg.clone().sex),
                avatar: Some(arg.clone().avatar),
                role_id: None,
                dept_id: None,
                remark: None,
                is_admin: None,
                phone_num: Some(arg.clone().phone),
                last_login_ip: None,
                last_login_time: None,
                created_at: None,
                updated_at: Some(DateTimeNative::now()),
                deleted_at: None
            };
            let res = tx.save::<UserEntity>(&user,&[]).await.expect("注册失败");
            if res.rows_affected != 0 {
                result = String::from("注册成功");
            }
        };
        tx.commit().await.expect("事务提交失败");
        return result;
    }

    /// 新增用户
    pub async fn add_user(req: AddUserReq){
        let uid = scru128::scru128().to_string();
        let salt = rand_string(10);
        let password = UserService::encrypt_password(&req.password,&salt);
        let user_entity = UserEntity{
            id: Some(uid),
            user_name: Some(req.user_name),
            user_nickname: Some(req.nickname),
            user_password: Some(password),
            user_salt: Some(salt),
            user_status: Some(req.status),
            user_email: req.email,
            sex: Some(req.gender),
            avatar: req.avatar,
            role_id: Some(req.role_id),
            dept_id: Some(req.dept_id),
            remark: req.remark,
            is_admin: Some(req.is_admin),
            phone_num: Some(req.phone),
            created_at: Some(DateTimeNative::now()),
            ..UserEntity::default() //后续的字段取默认值
        };
        let rb = init_rbatis().await;


    }
}


