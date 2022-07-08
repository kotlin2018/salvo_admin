use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

// 角色表
#[crud_table(table_name:role)]
#[derive(Debug,Serialize,Deserialize)]
pub struct RoleEntity {
    pub role_id: Option<u32>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub list_order: Option<i32>,
    pub data_scope: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Default for RoleEntity {
    fn default() -> Self {
        Self{
            role_id: None,
            role_name: None,
            role_key: None,
            list_order: None,
            data_scope: None,
            status: None,
            remark: None,
            created_at: None,
            updated_at: None
        }
    }
}