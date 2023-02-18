use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysDept {
    // 部门编码
    pub dept_id: Option<i32>,
    // 上级部门
    pub parent_id: Option<i32>,
    //
    pub dept_path: Option<String>,
    // 部门名称
    pub dept_name: Option<String>,
    // 排序
    pub sort: Option<i32>,
    // 负责人
    pub leader: Option<String>,
    // 手机
    pub phone: Option<String>,
    // 邮箱
    pub email: Option<String>,
    // 状态
    pub status: Option<i8>,

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
crud!(SysDept{});