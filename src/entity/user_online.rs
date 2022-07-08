use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_user_online)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserOnlineEntity {
    pub id: Option<Uuid>,
    pub u_id: Option<Uuid>,
    pub token_id: Option<Uuid>,
    pub token_exp: Option<i64>,
    pub login_time: Option<DateTimeNative>,
    pub user_name: Option<String>,
    pub dept_name: Option<String>,
    pub net: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub device: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
}

impl Default for UserOnlineEntity {
    fn default() -> Self {
        Self{
            id: None,
            u_id: None,
            token_id: None,
            token_exp: None,
            login_time: None,
            user_name: None,
            dept_name: None,
            net: None,
            ipaddr: None,
            login_location: None,
            device: None,
            browser: None,
            os: None
        }
    }
}