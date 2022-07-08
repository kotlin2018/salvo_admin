use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:dict_data)]
#[derive(Debug,Serialize,Deserialize)]
pub struct DictDataEntity {
    pub dict_data_id: Option<u32>,
    pub dict_sort: Option<i32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<bool>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for DictDataEntity {
    fn default() -> Self {
        Self{
            dict_data_id: None,
            dict_sort: None,
            dict_label: None,
            dict_value: None,
            dict_type: None,
            css_class: None,
            list_class: None,
            is_default: None,
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