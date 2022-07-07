use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Debug,Serialize,Deserialize)]
pub struct Role {
    pub id: u32,
    pub user_id: u32,
    pub role_id: u32,
    pub created_by: String,
    pub created_at: DateTimeNative,
}