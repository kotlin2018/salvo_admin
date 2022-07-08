use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_user_dept)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserDeptEntity {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub dept_id: Option<Uuid>,
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