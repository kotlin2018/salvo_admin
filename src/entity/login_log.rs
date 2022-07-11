use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_login_log)]
#[derive(Debug,Serialize,Deserialize)]
pub struct LoginLogEntity {
    pub info_id: Option<String>,
    pub login_name: Option<String>,
    pub net: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<DateTimeNative>,
    pub module: Option<String>,
}

impl Default for LoginLogEntity {
    fn default() -> Self {
        Self{
            info_id: None,
            login_name: None,
            net: None,
            ipaddr: None,
            login_location: None,
            browser: None,
            os: None,
            device: None,
            status: None,
            msg: None,
            login_time: None,
            module: None
        }
    }
}