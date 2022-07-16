use rbatis::{DateTimeNative, Page, PageRequest, Uuid};
use rbatis::executor::RbatisExecutor;
use rbatis::rbatis::Rbatis;
use serde::{Serialize, Deserialize};
use crate::dto::request_data::SearchReq;
use crate::dto::response_data::UserAndDeptResp;
use crate::entity::user_dept::UserDeptEntity;

// 用户表
// #[crud_table(table_name:"sys_user" | formats_mysql:"id:{}::uuid")] //此处要注释掉这行
#[crud_table(table_name:"sys_user")]
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct UserEntity {
    // pub id: Option<Uuid>, // 使用Uuid会报错，推荐使用 String
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub user_nickname: Option<String>,
    pub user_password: Option<String>,
    pub user_salt: Option<String>,
    pub user_status: Option<String>,
    pub user_email: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    // pub role_id: Option<Uuid>,
    // pub dept_id: Option<Uuid>,
    pub role_id: Option<String>,
    pub dept_id: Option<String>,
    pub remark: Option<String>,
    pub is_admin: Option<bool>,
    pub phone_num: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<DateTimeNative>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub deleted_at: Option<DateTimeNative>,
}

impl Default for UserEntity {
    fn default() -> Self {
       Self{
           id: None,
           user_name: None,
           user_nickname: None,
           user_password: None,
           user_salt: None,
           user_status: None,
           user_email: None,
           sex: None,
           avatar: None,
           role_id: None,
           dept_id: None,
           remark: None,
           is_admin: None,
           phone_num: None,
           last_login_ip: None,
           last_login_time: None,
           created_at: None,
           updated_at: None,
           deleted_at: None
       }
    }
}

impl UserEntity {
    #[html_sql("./src/mapper/user_mapper.html")]
    pub async fn get_sort_list(rb: &mut RbatisExecutor<'_,'_>,page_req: &PageRequest,sql_arg: &SearchReq) ->Result<Page<UserAndDeptResp>,rbatis::Error>{impled!()}

    #[py_sql("select * from sys_user where user_name =#{name}")]
    pub async fn fetch_one(rb: &Rbatis,name: &str) ->Result<UserEntity,rbatis::Error>{impled!()}

    #[html_sql("./src/mapper/user_mapper.html")]
    pub async fn get_by_id(rb: &Rbatis,user_id: &str) ->Result<UserAndDeptResp,rbatis::Error> {impled!()}

    #[html_sql("./src/mapper/user_mapper.html")]
    pub async fn get_dept(rb: &Rbatis,dept_id: &str) ->Result<UserDeptEntity,rbatis::Error> {impled!()}

}

#[cfg(test)]
mod test{
    use rbatis::crud::CRUD;
    use rbatis::{DateTimeNative, PageRequest, Uuid};
    use crate::dto::request_data::SearchReq;
    use crate::entity::user::UserEntity;
    use crate::init_rbatis;

    #[tokio::test]
    async fn test_insert(){
        let rb = init_rbatis().await;
        let user = UserEntity{
            //id: Some(Uuid::new()),
            id: Some(Uuid::new().to_string()),
            user_name: Some("admin".to_string()),
            user_nickname: Some("马磊".to_string()),
            user_password: Some("123456".to_string()),
            user_salt: Some("4#)MGcEj2p".to_string()),
            user_status: Some("1".to_string()),
            user_email: Some("123456789@136.com".to_string()),
            sex: Some("男".to_string()),
            avatar: Some("/upload/2022-03/10-0105B3BSNSQ6LVEEG4GG3UU1KI.jpg".to_string()),
            role_id: None,
            dept_id: None,
            remark: None,
            is_admin: None,
            phone_num: None,
            last_login_ip: None,
            last_login_time: None,
            created_at: Some(DateTimeNative::now()),
            updated_at: None,
            deleted_at: None
        };
        let r = rb.save(&user,&[]).await;
        if r.is_err() {
            println!("{}",r.err().unwrap().to_string())
        }
    }

    #[tokio::test]
    async fn test_wrapper(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb = init_rbatis().await;
        let user_name = "admin";
        let w = rb.new_wrapper().eq("user_name", &user_name);
        let user = rb.fetch_list_by_wrapper::<UserEntity>(w).await.unwrap();
        println!("{:?}",user);
    }

    #[tokio::test]
    async fn test_py_sql(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb = init_rbatis().await;
        let user = UserEntity::fetch_one(&rb,"admin").await;
        println!("user = {:?}",user);
        let user2 = UserEntity::fetch_one(&rb,"admin").await;
        println!("user2 = {:?}",user2);
    }

    #[tokio::test]
    async fn test_get_by_id(){
        fast_log::init(fast_log::config::Config::new().console()).expect("TODO: panic message");
        let rb =init_rbatis().await;
        let user_dept = UserEntity::get_by_id(&rb,"00TV87DDOBJPU75J4TGUOC3NNG").await;
        println!("user_dept = {:?}",user_dept);
        println!("================================================================");
        let user_dept2 = UserEntity::get_by_id(&rb,"00TV87DDOBJPU75J4TGUOC3NNG").await;
        println!("user_dept2 = {:?}",user_dept2);
    }
}

