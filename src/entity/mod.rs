pub(crate) mod casbin_rule;
pub(crate) mod data_scope;
pub(crate) mod sys_api;
pub(crate) mod sys_config;
pub(crate) mod sys_dept;
pub(crate) mod sys_dict_data;
pub(crate) mod sys_dict_type;
pub(crate) mod sys_login_log;
pub(crate) mod sys_menu;
pub(crate) mod sys_opera_log;
pub(crate) mod sys_post;
pub(crate) mod sys_role;
pub(crate) mod sys_user;

#[test]
pub fn test_entity(){
    // 在 entity mod 中调用 handler mod 中的函数
    use crate::handler::*;
    test_handle2();
}

pub fn test_entity2(){
    println!("entity2");
}