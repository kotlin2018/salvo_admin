use serde::Deserialize;

#[derive(Debug,Clone,Deserialize)]
pub struct DataPermission{
    pub data_scope: Option<String>,
    pub user_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub role_id: Option<i32>,
}