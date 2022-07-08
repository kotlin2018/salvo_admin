use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:dict_type)]
#[derive(Debug,Serialize,Deserialize)]
pub struct DictTypeEntity {
    pub dict_type_id: Option<u32>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for DictTypeEntity {
    fn default() -> Self {
        Self{
            dict_type_id: None,
            dict_name: None,
            dict_type: None,
            status: None,
            create_by: None,
            update_by: None,
            remark: None,
            created_at: None,
            updated_at: None,
            deleted_at: None
        }
    }
}