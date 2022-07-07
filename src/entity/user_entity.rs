use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub id: Option<u32>,
    pub user_name: Option<String>,
    pub user_nickname: Option<String>,
    pub user_password: Option<String>,
    pub user_salt: Option<String>,
    pub user_email: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<u32>,
    pub dept_id: Option<u32>,
    pub remark: Option<String>,
    pub is_admin: Option<bool>,
    pub phone_num: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<DateTimeNative>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for User {
    fn default() -> Self {
       Self{
           id: None,
           user_name: None,
           user_nickname: None,
           user_password: None,
           user_salt: None,
           user_email: None,
           sex: None,
           avatar: None,
           role_id: None,
           dept_id: None,
           remark: None,
           is_admin: None,
           phone_num: None,
           last_login_ip: None,
           last_login_time: None,
           created_at: None,
           updated_at: None,
           deleted_at: None
       }
    }
}