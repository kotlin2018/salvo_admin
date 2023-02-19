use std::fmt::Debug;
use std::io::Read;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONTEXT: Context = Context::default();
    pub static ref CONFIG: Settings = Settings::default();
}

#[derive(Debug)]
pub struct Context{
    pub db: rbatis::Rbatis,
    pub redis: Option<redis::Client>,
}

#[derive(Debug,Deserialize,Clone)]
pub struct Settings {
    pub application: Application,
    pub logger: Logger,
    pub jwt: JWT,
    pub database: Database,
    pub redis: Redis,
}

#[derive(Debug,Deserialize,Clone)]
pub struct Application{
    pub mode: Option<String>,
    pub host: String,
    pub name: Option<String>,
    pub port: String,
    pub read_timeout: Option<String>,
    pub writer_timeout: Option<String>,
    pub enable_dp: Option<bool>,
}
#[derive(Debug,Deserialize,Clone)]
pub struct Logger {
    pub path: Option<String>,
    pub stdout: Option<String>,
    pub level: Option<String>,
    pub enable_db: Option<bool>,
}
#[derive(Debug,Deserialize,Clone)]
pub struct JWT {
    pub secret: String,
    pub timeout: usize,
}
#[derive(Debug,Deserialize,Clone)]
pub struct Database {
    pub driver: Option<String>,
    pub source: Option<String>,
}
#[derive(Debug,Deserialize,Clone)]
pub struct Redis {
    pub addr: Option<String>,
    pub password: Option<String>,
    pub db: Option<String>
}

impl Default for Settings {
    fn default() -> Self {
        let yaml_data = std::fs::read_to_string("src/config/salvo-admin.yaml").unwrap();
        //let yaml_data = include_str!("salvo-admin.yaml");
        serde_yaml::from_str::<Settings>(&yaml_data).unwrap()
    }
}

impl Default for Context {
    fn default() -> Self {
        let db = Rbatis::new();

        let setting = Settings::default();
        let database_link = setting.database.source.unwrap().clone();
        let redis_url = setting.redis.addr.unwrap().clone();

        db.init(MysqlDriver{},&database_link)
            .expect("rbatis link database fail!");

        // {
        //     db.init(
        //         rbdc_sqlite::driver::SqliteDriver {},
        //         "sqlite://tests/sqlite.db",
        //     ).unwrap();
        // }

        let mut context = Context{
            db,
            redis: Some(redis::Client::open(redis_url).unwrap()),
        };
        context
    }
}

#[tokio::test]
async fn test_settings(){
    use crate::entity::sys_user::SysUser;
    let rb = &CONTEXT.db;
    // let sql = std::fs::read_to_string("tests/table_sqlite.sql").unwrap();
    // rb.exec(&sql, vec![]).await.expect("TODO: panic message");
    let username = rbs::Value::String("admin".to_string());
    let password = rbs::Value::String("123456".to_string());

    let value = rb.query_decode::<SysUser>("select * from sys_user where username = ? and password = ?",vec![username,password]).await;
    println!("value = {:?}",value);
}






