use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:post)]
#[derive(Debug,Serialize,Deserialize)]
pub struct PostEntity {
    pub post_id: Option<u32>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<i32>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for PostEntity {
    fn default() -> Self {
        Self{
            post_id: None,
            post_code: None,
            post_name: None,
            post_sort: None,
            status: None,
            remark: None,
            created_by: None,
            updated_by: None,
            created_at: None,
            updated_at: None,
            deleted_at: None
        }
    }
}