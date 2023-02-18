use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysUser {
    // 编码
    pub user_id: Option<i32>,
    // 用户名
    pub username: Option<String>,
    // 密码
    pub password: Option<String>,
    // 昵称
    pub nickname: Option<String>,
    // 手机号
    pub phone: Option<String>,
    // 角色ID
    pub role_id: Option<i32>,
    // 加盐
    pub salt: Option<String>,
    // 头像
    pub avatar: Option<String>,
    // 性别
    pub sex: Option<String>,
    // 邮箱
    pub email: Option<String>,
    // 部门
    pub dept_id: Option<i32>,
    // 岗位
    pub post_id: Option<i32>,
    // 备注
    pub remark: Option<String>,
    // 状态
    pub status: Option<String>,
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
crud!(SysUser{});