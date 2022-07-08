use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_role_dept)]
#[derive(Debug,Serialize,Deserialize)]
pub struct RoleDeptEntity {
    pub role_id: Option<Uuid>,
    pub dept_id: Option<Uuid>,
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