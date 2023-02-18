use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysRole {
    // 角色编码
    pub role_id: Option<i32>,
    // 角色名称
    pub role_name: Option<String>,

    pub status: Option<String>,
    // 角色代码
    pub role_key: Option<String>,
    // 角色排序
    pub role_sort: Option<i8>,
    //
    pub flag: Option<String>,
    // 备注
    pub remark: Option<String>,
    pub admin: Option<bool>,
    pub data_scope: Option<String>,
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
crud!(SysRole{});

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysRoleDept {
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
}
crud!(SysRoleDept{});

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysRoleMenu {
    pub role_id: Option<i32>,
    pub menu_id: Option<i32>,
}
crud!(SysRoleMenu{});