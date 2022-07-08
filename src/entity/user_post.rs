use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:user_post)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserPostEntity {
    pub user_id: Option<u32>,
    pub post_id: Option<u32>,
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