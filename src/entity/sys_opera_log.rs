use rbatis::rbdc::datetime::FastDateTime;
use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SysOperaLog {
    pub id: Option<i32>,
    // 操作模块
    pub title: Option<String>,
    // 操作类型
    pub business_type: Option<String>,
    //
    pub business_types: Option<String>,
    // 函数
    pub method: Option<String>,
    // 请求方式 GET POST PUT DELETE
    pub request_method: Option<String>,
    // 操作类型
    pub operator_type: Option<String>,
    // 操作者
    pub oper_name: Option<String>,
    // 部门名称
    pub dept_name: Option<String>,
    // 访问地址
    pub oper_url: Option<String>,
    // 客户端ip
    pub oper_ip: Option<String>,
    // 访问位置
    pub oper_location: Option<String>,
    // 请求参数
    pub oper_param: Option<String>,
    // 操作状态 1:正常 2:关闭
    pub status: Option<String>,
    // 操作时间
    pub oper_time: Option<FastDateTime>,
    // 返回数据
    pub json_result: Option<String>,
    // 备注
    pub remark: Option<String>,
    // 耗时
    pub latency_time: Option<String>,
    // userAgent
    pub user_agent: Option<String>,
    // 创建者
    pub create_by: Option<i32>,
    // 更新者
    pub update_by: Option<i32>,
    // 创建时间
    pub created_at: Option<FastDateTime>,
    // 最后更新时间
    pub updated_at: Option<FastDateTime>,
}
crud!(SysOperaLog{});