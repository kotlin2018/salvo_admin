use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysConfig {
    // 主键编码
    pub id: Option<i32>,
    //
    pub config_name: Option<String>,
    //
    pub config_key: Option<String>,
    //
    pub config_value: Option<String>,
    //
    pub config_type: Option<String>,
    // 是否前台
    pub is_fronted: Option<String>,
    //
    pub remark: Option<String>,

    // 创建者
    pub create_by: Option<i32>,
    // 更新者
    pub update_by: Option<i32>,
    // 创建时间
    pub created_at: Option<FastDateTime>,
    // 最后更新时间
    pub updated_at: Option<FastDateTime>,
    // 删除时间
    pub deleted_at: Option<FastDateTime>,
}

crud!(SysConfig{});