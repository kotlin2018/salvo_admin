use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug,Serialize,Deserialize)]
pub struct Role {
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub role_id: Option<u32>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTimeNative>,
}

impl Default for Role {
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