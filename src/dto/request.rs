use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct SysUserGetPageReq {
    #[serde(rename="pageIndex")]
    pub page_index: Option<i32>,
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,

    #[serde(rename="userId")]
    pub user_id: Option<i32>,
    pub username: Option<String>,
    #[serde(rename="nickName")]
    pub nickname: Option<String>,
    pub phone: Option<String>,
    #[serde(rename="roleId")]
    pub role_id: Option<String>,
    pub sex: Option<String>,
    pub email: Option<String>,
    #[serde(rename="postId")]
    pub post_id: Option<String>,
    pub status: Option<String>,

    #[serde(rename="deptId")]
    pub dept_id: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
    pub code: String,
    pub uuid: String,
}