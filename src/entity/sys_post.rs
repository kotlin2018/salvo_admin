use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;

#[derive(Debug)]
pub struct SysPost {
    // 岗位编号
    pub post_id: Option<i32>,
    // 岗位名称
    pub post_name: Option<String>,
    // 岗位代码
    pub post_code: Option<String>,
    // 岗位排序
    pub sort: Option<i32>,
    // 状态
    pub status: Option<i32>,
    // 描述
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
crud!(SysPost{});