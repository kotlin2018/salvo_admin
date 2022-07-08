use rbatis::DateTimeNative;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:job)]
#[derive(Debug,Serialize,Deserialize)]
pub struct JobEntity {
    pub job_id: Option<u32>,
    pub task_id: Option<u32>,
    pub task_count: Option<u64>,
    pub run_count: Option<u64>,
    pub job_name: Option<String>,
    pub job_params: Option<String>,
    pub job_group: Option<String>,
    pub invoke_target: Option<String>,
    pub cron_expression: Option<String>,
    pub misfire_policy: Option<String>,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub last_time: Option<DateTimeNative>,
    pub next_time: Option<DateTimeNative>,
    pub end_time: Option<DateTimeNative>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for JobEntity {
    fn default() -> Self {
        Self{
            job_id: None,
            task_id: None,
            task_count: None,
            run_count: None,
            job_name: None,
            job_params: None,
            job_group: None,
            invoke_target: None,
            cron_expression: None,
            misfire_policy: None,
            concurrent: None,
            status: None,
            create_by: None,
            update_by: None,
            remark: None,
            last_time: None,
            next_time: None,
            end_time: None,
            created_at: None,
            updated_at: None,
            deleted_at: None
        }
    }
}