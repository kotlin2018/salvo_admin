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

