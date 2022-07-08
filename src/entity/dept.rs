use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_dept)]
#[derive(Debug,Serialize,Deserialize)]
pub struct DeptEntity {
    pub dept_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub dept_name: Option<String>,
    pub order_num: Option<u32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for DeptEntity {
    fn default() -> Self {
        Self{
            dept_id: None,
            parent_id: None,
            dept_name: None,
            order_num: None,
            leader: None,
            phone: None,
            email: None,
            status: None,
            created_by: None,
            updated_by: None,
            created_at: None,
            updated_at: None,
            deleted_at: None
        }
    }
}