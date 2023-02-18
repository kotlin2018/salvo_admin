use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysMenu {
    pub menu_id: Option<i32>,
    pub menu_name: Option<String>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub paths: Option<String>,
    pub menu_type: Option<String>,
    pub action: Option<String>,
    pub permission: Option<String>,
    pub parent_id: Option<i32>,
    pub no_cache: Option<bool>,
    pub breadcrumb: Option<String>,
    pub component: Option<String>,
    pub sort: Option<i8>,
    pub visible: Option<String>,
    pub is_frame: Option<String>,
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
crud!(SysMenu{});

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysMenuApiRule {
    pub sys_menu_menu_id: Option<i64>,
    pub sys_api_id: Option<i64>,
}

crud!(SysMenuApiRule{});