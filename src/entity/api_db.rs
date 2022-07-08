use rbatis::Uuid;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_api_db)]
#[derive(Debug,Serialize,Deserialize)]
pub struct ApiDBEntity {
    pub api_id: Option<Uuid>,
    pub db: Option<String>,
}

impl Default for ApiDBEntity {
    fn default() -> Self {
        Self{ api_id: None, db: None }
    }
}