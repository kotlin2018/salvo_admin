use rbatis::{executor::Executor, impled,Result};
use crate::entity::sys_role::SysRole;
use crate::entity::sys_user::SysUser;

// #[html_sql("mapper/salvo_admin.xml")]
// pub async fn permission(rb: &mut dyn Executor,data_scope: i32,table_name: &str,dept_path: Option<&str>,user_id: Option<i32>,role_id: Option<i32>, dept_id: Option<i32>){todo!()}

#[html_sql("mapper/salvo_admin.html")]
pub async fn get_user(rb: &mut dyn Executor,username: String,password: String) -> rbatis::Result<SysUser>{impled!()}


#[html_sql("mapper/salvo_admin.html")]
pub async fn get_role(rb: &mut dyn Executor,role_id: i32) -> rbatis::Result<SysRole>{impled!()}

#[cfg(test)]
mod tests {
    use rbatis::rbdc::datetime::FastDateTime;
    use super::*;
    use crate::config::CONTEXT;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<FastDateTime>,
        pub version: Option<i64>,
        pub delete_flag: Option<i32>,
    }


    #[html_sql("mapper/example.html")]
    async fn select_by_condition(
        rb: &mut dyn Executor,
        name: &str,
        dt: &FastDateTime,
    ) -> rbatis::Result<Vec<BizActivity>> {
        impled!()
    }

    #[tokio::test]
    async fn test_example_html_sql(){
        let mut rb = &CONTEXT.db;
        let a = select_by_condition(&mut rb.clone(), "test", &FastDateTime::now().set_micro(0))
            .await
            .unwrap();
        println!("{:?}", a);
    }

    #[html_sql("mapper/example.html")]
    async fn get_obj(rb: &mut dyn Executor) -> rbatis::Result<Vec<BizActivity>>{impled!()}

    #[tokio::test]
    async fn test_example_html_sql2(){
        let mut rb = &CONTEXT.db;
        let a = get_obj(&mut rb)
            .await
            .unwrap();
        println!("{:?}", a);
    }


    #[tokio::test]
    async fn test_html_file_sql(){
        let mut rb = &CONTEXT.db;
        let user = get_user(&mut rb,"admin".to_string(),"$2a$10$/Glr4g9Svr6O0kvjsRJCXu3f0W8/dsP3XZyVNi1019ratWpSPMyw.".to_string()).await.unwrap();
        println!("{user:?}")
    }
}