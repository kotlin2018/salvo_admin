use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub id: u32,
    pub user_name: String,
    pub user_nickname: String,
    pub user_password: String,
    pub user_salt: String,
    pub user_email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub role_id: u32,
    pub dept_id: u32,
    pub remark: Option<String>,
    pub is_admin: bool,
    pub phone_num: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<DateTimeNative>,
    pub created_at: DateTimeNative,
    pub updated_at: DateTimeNative,
    pub deleted_at: DateTimeNative,
}