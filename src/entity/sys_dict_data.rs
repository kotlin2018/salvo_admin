use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;

#[derive(Debug)]
pub struct SysDictData {
    // 主键编码
    pub dict_code: Option<i32>,
    pub dict_sort: Option<i32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<String>,
    pub status: Option<i8>,
    pub default: Option<String>,
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
crud!(SysDictData{});