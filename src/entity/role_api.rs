use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:role_api)]
#[derive(Debug,Serialize,Deserialize)]
pub struct RoleApiEntity {
    pub id: Option<u32>,
    pub role_id: Option<u32>,
    pub api: Option<String>,
    pub method: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTimeNative>,
}

impl Default for RoleApiEntity {
    fn default() -> Self {
        Self{
            id: None,
            role_id: None,
            api: None,
            method: None,
            created_by: None,
            created_at: None
        }
    }
}