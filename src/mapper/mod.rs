use rbatis::{executor::Executor, impled,Result};
use crate::entity::sys_dept::SysDept;
use crate::entity::sys_role::SysRole;
use crate::entity::sys_user::SysUser;

// #[html_sql("mapper/salvo_admin.xml")]
// pub async fn permission(rb: &mut dyn Executor,data_scope: i32,table_name: &str,dept_path: Option<&str>,user_id: Option<i32>,role_id: Option<i32>, dept_id: Option<i32>){todo!()}

#[html_sql("src/mapper/salvo_admin.html")]
pub async fn get_user(rb: &mut dyn Executor,username: String,password: String,page_no:i32,page_size: i32) -> rbatis::Result<Vec<SysUser>>{impled!()}


#[html_sql("src/mapper/salvo_admin.html")]
pub async fn get_role(rb: &mut dyn Executor,role_id: i32) -> rbatis::Result<SysRole>{impled!()}

#[html_sql("src/mapper/salvo_admin.html")]
pub async fn get_dept(rb: &mut dyn Executor,dept_id: i32) -> Result<SysDept>{impled!()}

#[cfg(test)]
mod tests {
    use rbatis::rbdc::datetime::FastDateTime;
    use super::*;
    use crate::config::CONTEXT;

    #[tokio::test]
    async fn test_html_file_sql(){
        let mut rb = &CONTEXT.db;
        let password = "$2a$10$/Glr4g9Svr6O0kvjsRJCXu3f0W8/dsP3XZyVNi1019ratWpSPMyw.".to_string();
        let user = get_user(&mut rb,"admin".to_string(),password,1,0).await.unwrap();
        println!("{user:?}")
    }
}