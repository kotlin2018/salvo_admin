use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysApi {
    // 主键编码
    pub id: Option<i32>,
    //
    pub handle: Option<String>,
    // 标题
    pub title: Option<String>,
    // 地址
    pub path: Option<String>,
    // 请求类型
    pub action: Option<String>,
    // 接口类型
    pub r#type: Option<String>,

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
crud!(SysApi{});