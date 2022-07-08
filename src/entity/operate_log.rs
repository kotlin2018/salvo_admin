use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:operate_log)]
#[derive(Debug,Serialize,Deserialize)]
pub struct OperateLogEntity {
    pub operate_id: Option<u32>,
    pub time_id: Option<u32>,
    pub title: Option<String>,
    pub business_type: Option<String>,
    pub method: Option<String>,
    pub request_method: Option<String>,
    pub operator_type: Option<String>,
    pub operate_name: Option<String>,
    pub dept_name: Option<String>,
    pub operate_url: Option<String>,
    pub operate_ip: Option<String>,
    pub operate_location: Option<String>,
    pub operate_param: Option<String>,
    pub path_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<String>,
    pub error_msg: Option<String>,
    pub duration: Option<i64>,
    pub operate_time: Option<DateTimeNative>,
}

impl Default for OperateLogEntity {
    fn default() -> Self {
        Self{
            operate_id: None,
            time_id: None,
            title: None,
            business_type: None,
            method: None,
            request_method: None,
            operator_type: None,
            operate_name: None,
            dept_name: None,
            operate_url: None,
            operate_ip: None,
            operate_location: None,
            operate_param: None,
            path_param: None,
            json_result: None,
            status: None,
            error_msg: None,
            duration: None,
            operate_time: None
        }
    }
}