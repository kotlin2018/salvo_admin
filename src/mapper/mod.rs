use rbatis::{executor::Executor,html_sql, impled,Result};
use crate::entity::sys_role::SysRole;
use crate::entity::sys_user::SysUser;

#[html_sql("salvo_admin.xml")]
pub fn permission(rb: &mut dyn Executor,table_name: &str,dept_path: Option<&str>,user_id: Option<i32>,role_id: Option<i32>, dept_id: Option<i32>){todo!()}

#[html_sql("salvo_admin.xml")]
pub async fn get_user(rb: &mut dyn Executor,username: String,password: String) -> Result<SysUser>{impled!()}


#[html_sql("salvo_admin.xml")]
pub async fn get_role(rb: &mut dyn Executor,role_id: i32) -> Result<SysRole>{impled!()}