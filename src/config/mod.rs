use std::io::Read;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONTEXT: Context = Context::default();
}

#[derive(Debug)]
pub struct Context{
    pub db: rbatis::Rbatis,
    pub redis: Option<redis::Client>,
}

#[derive(Debug,Deserialize)]
pub struct Settings {
    pub application: Application,
    pub logger: Logger,
    pub jwt: JWT,
    pub database: Database,
    pub redis: Redis,
}

#[derive(Debug,Deserialize)]
pub struct Application{
    pub mode: Option<String>,
    pub host: Option<String>,
    pub name: Option<String>,
    pub port: Option<String>,
    pub read_timeout: Option<String>,
    pub writer_timeout: Option<String>,
    pub enable_dp: Option<bool>,
}
#[derive(Debug,Deserialize)]
pub struct Logger {
    pub path: Option<String>,
    pub stdout: Option<String>,
    pub level: Option<String>,
    pub enable_db: Option<bool>,
}
#[derive(Debug,Deserialize)]
pub struct JWT {
    pub secret: Option<String>,
    pub timeout: Option<String>,
}
#[derive(Debug,Deserialize)]
pub struct Database {
    pub driver: Option<String>,
    pub source: Option<String>,
}
#[derive(Debug,Deserialize)]
pub struct Redis {
    pub addr: Option<String>,
    pub password: Option<String>,
    pub db: Option<String>
}

impl Default for Context {
    fn default() -> Self {
        let yaml_data = std::fs::read_to_string("src/config/salvo-admin.yaml").unwrap();
        //let yaml_data = include_str!("salvo-admin.yaml");
        let settings = serde_yaml::from_str::<Settings>(&yaml_data).unwrap();
        let db = Rbatis::new();
        let database_link = settings.database.source.unwrap();
        db.init(MysqlDriver{},&database_link)
            .expect("rbatis link database fail!");
        let mut context = Context{
            db,
            redis: Some(redis::Client::open(settings.redis.addr.unwrap().as_str()).unwrap()),
        };
        context
    }
}

#[tokio::test]
async fn test_settings(){
    let rb = &CONTEXT.db;
    let value = rb.fetch("select * from sys_user",vec![]).await;
    println!("value = {:?}",value);
}
