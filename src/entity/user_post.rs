use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_user_post)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserPostEntity {
    pub user_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub created_at: Option<DateTimeNative>
}

impl Default for UserPostEntity {
    fn default() -> Self {
        Self{
            user_id: None,
            post_id: None,
            created_at: None
        }
    }
}