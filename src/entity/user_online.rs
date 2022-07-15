use rbatis::{DateTimeNative, Uuid};
use rbatis::db::DBExecResult;
use rbatis::rbatis::Rbatis;
use serde::{Serialize, Deserialize};

#[crud_table(table_name:sys_user_online)]
#[derive(Debug,Serialize,Deserialize)]
pub struct UserOnlineEntity {
    pub id: Option<String>,
    pub u_id: Option<String>,
    pub token_id: Option<String>,
    pub token_exp: Option<i64>,
    pub login_time: Option<DateTimeNative>,
    pub user_name: Option<String>,
    pub dept_name: Option<String>,
    pub net: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub device: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
}

impl Default for UserOnlineEntity {
    fn default() -> Self {
        Self{
            id: None,
            u_id: None,
            token_id: None,
            token_exp: None,
            login_time: None,
            user_name: None,
            dept_name: None,
            net: None,
            ipaddr: None,
            login_location: None,
            device: None,
            browser: None,
            os: None
        }
    }
}

impl UserOnlineEntity {
    #[py_sql("
          update sys_user_online set token_exp=#{token_exp} where token_id =#{token_id}
    ")]
    pub async fn update_online(rb: &Rbatis,token_id: &str,token_exp: &u64) -> Result<DBExecResult,rbatis::Error>{impled!()}
}

