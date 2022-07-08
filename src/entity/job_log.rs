use rbatis::{DateTimeNative, Uuid};
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_job_log)]
#[derive(Debug,Serialize,Deserialize)]
pub struct JobLogEntity {
    pub job_log_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub lot_id: Option<u32>,
    pub lot_order: Option<i64>,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub job_params: Option<String>,
    pub job_message: Option<String>,
    pub invoke_target: Option<String>,
    pub status: Option<String>,
    pub exception_info: Option<String>,
    pub is_once: Option<bool>,
    pub created_at: Option<DateTimeNative>,
    pub elapsed_time: Option<i64>,
}

impl Default for JobLogEntity {
    fn default() -> Self {
        Self{
            job_log_id: None,
            job_id: None,
            lot_id: None,
            lot_order: None,
            job_name: None,
            job_group: None,
            job_params: None,
            job_message: None,
            invoke_target: None,
            status: None,
            exception_info: None,
            is_once: None,
            created_at: None,
            elapsed_time: None
        }
    }
}