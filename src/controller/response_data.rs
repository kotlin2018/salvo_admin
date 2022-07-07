use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct UserResp {
    pub id: u32,
    pub user_name: Option<String>,
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    pub user_email: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub dept_id: Option<u32>,
    pub role_id: Option<u32>,
    pub remark: Option<String>,
    pub is_admin: Option<bool>,
    pub phone_num: Option<String>,
    pub created_at: Option<DateTimeNative>,
}