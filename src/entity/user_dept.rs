use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:user_dept)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserDeptEntity {
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub dept_id: Option<u32>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTimeNative>,
}

impl Default for UserDeptEntity {
    fn default() -> Self {
        Self{
            id: None,
            user_id: None,
            dept_id: None,
            created_by: None,
            created_at: None
        }
    }
}