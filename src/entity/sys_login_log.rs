use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;

#[derive(Debug)]
pub struct SysLoginLog {
    pub id: Option<i32>,
    // 用户名
    pub username: Option<String>,
    // 状态
    pub status: Option<String>,
    // ip地址
    pub ipaddr: Option<String>,
    //
    pub login_location: Option<String>,
    //
    pub browser: Option<String>,
    //
    pub os: Option<String>,
    //
    pub platform: Option<String>,
    //
    pub login_time: Option<FastDateTime>,
    //
    pub remark: Option<String>,
    //
    pub msg: Option<String>,
    //
    // 创建者
    pub create_by: Option<i32>,
    // 更新者
    pub update_by: Option<i32>,
    // 创建时间
    pub created_at: Option<FastDateTime>,
    // 最后更新时间
    pub updated_at: Option<FastDateTime>,
}
crud!(SysLoginLog{});