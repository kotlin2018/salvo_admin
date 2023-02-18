use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DataPermission{
    pub data_scope: Option<String>,
    pub user_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub role_id: Option<i32>,
}
crud!(DataPermission{});