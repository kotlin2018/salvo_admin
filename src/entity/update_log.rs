use rbatis::{DateTimeNative, Uuid};
use salvo::http::headers::Date;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_update_log)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateLogEntity {
    pub id: Option<Uuid>,
    pub app_version: Option<String>,
    pub backend_version: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
    pub updated_by: Option<String>,
}

impl Default for UpdateLogEntity {
    fn default() -> Self {
        Self{
            id: None,
            app_version: None,
            backed_version: None,
            title: None,
            content: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            updated_by: None
        }
    }
}
