use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_menu)]
#[derive(Debug,Serialize,Deserialize)]
pub struct MenuEntity {
    pub id: Option<Uuid>,
    pub pid: Option<Uuid>,
    pub path: Option<String>,
    pub menu_name: Option<String>,
    pub icon: Option<String>,
    pub menu_type: Option<String>,
    pub query: Option<String>,
    pub order_sort: Option<u32>,
    pub status: Option<String>,
    pub api : Option<String>,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: Option<String>,
    pub is_cache: Option<String>,
    pub log_method: Option<String>,
    pub data_cache_method: Option<String>,
    pub is_frame: Option<String>,
    pub data_scope: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for MenuEntity {
    fn default() -> Self {
        Self{
            id: None,
            pid: None,
            path: None,
            menu_name: None,
            icon: None,
            menu_type: None,
            query: None,
            order_sort: None,
            status: None,
            api: None,
            method: None,
            component: None,
            visible: None,
            is_cache: None,
            log_method: None,
            data_cache_method: None,
            is_frame: None,
            data_scope: None,
            remark: None,
            created_at: None,
            updated_at: None,
            deleted_at: None
        }
    }
}