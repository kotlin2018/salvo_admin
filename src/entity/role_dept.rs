use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:role_dept)]
#[derive(Debug,Serialize,Deserialize)]
pub struct RoleDeptEntity {
    pub role_id: Option<u32>,
    pub dept_id: Option<u32>,
    pub created_at: Option<DateTimeNative>,
}

impl Default for RoleDeptEntity {
    fn default() -> Self {
        Self{
            role_id: None,
            dept_id: None,
            created_at: None
        }
    }
}