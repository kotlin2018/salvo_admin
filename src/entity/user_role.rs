use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_user_role)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserRoleEntity {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTimeNative>,
}

impl Default for UserRoleEntity {
    fn default() -> Self {
        Self{
            id: None,
            user_id: None,
            role_id: None,
            created_by: None,
            created_at: None
        }
    }
}