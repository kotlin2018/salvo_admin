use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysDictType {
    // 主键编码
    pub id: Option<i32>,
    //
    pub dict_name: Option<String>,
    //
    pub dict_type: Option<String>,
    //
    pub status: Option<i8>,
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
crud!(SysDictType{});