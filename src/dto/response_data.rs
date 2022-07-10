use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct UserAndDeptResp {
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    pub user_email: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<u32>,
    pub remark: Option<String>,
    pub is_admin: Option<bool>,
    pub phone_num: Option<String>,
    pub created_at: Option<DateTimeNative>,

    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CaptchaImageResp {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AuthBodyResp {
    pub token: String,
    pub token_type: String,
    pub exp: i64,
    pub exp_in: i64,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ClientNetResp {
    pub ip: String,
    pub location: String,
    pub net_work: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct UserAgentResp{
    pub browser: String,
    pub os: String,
    pub device: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ClientResp {
    pub net: ClientNetResp,
    pub ua: UserAgentResp,
}