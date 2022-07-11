pub mod user_mapper;

use once_cell::sync::Lazy;
use rbatis::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
use yaml_rust::{Yaml, YamlLoader};

// 服务启动配置
pub struct ApplicationConfig{
    pub debug: bool,
    pub server_url: String,
    pub redis_url: String,
    pub database_url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    pub log_dir: String,
    pub log_cup: i64,
    pub log_temp_size: String,
    pub log_zip: bool,
    pub log_rolling_type: String,
    pub log_level: String,
    pub sms_redis_send_key_prefix: String,
    pub jwt_secret: String,
    pub jwt_expire: i64,
    pub white_list_api: Vec<String>,
    pub cache_type: String,
    // 登陆失败重试
    pub login_fail_replay: i64,
    pub login_fail_retry_wait_sec: i64,
    // 超级管理员账号
    pub super_user: Vec<String>,
    // user agent 解析
    pub user_agent_parser: String,
}

// 默认配置(默认实现)
impl Default for ApplicationConfig{
    fn default() -> Self {
        let yaml_data = include_str!("../../application.yml");
        let docs = YamlLoader::load_from_str(yaml_data).unwrap();
        // 读取配置
        let result = Self{
            debug: get_cfg(&docs,"debug").as_bool().unwrap_or(true),
            server_url: get_cfg(&docs,"server_url").as_str().unwrap_or("").to_owned(),
            redis_url: get_cfg(&docs,"redis_url").as_str().unwrap_or("").to_owned(),
            database_url: get_cfg(&docs,"database_url").as_str().unwrap_or("").to_owned(),
            logic_column: get_cfg(&docs,"logic_column").as_str().unwrap_or("").to_owned(),
            logic_un_deleted: get_cfg(&docs,"logic_un_deleted").as_i64().unwrap_or_default(),
            logic_deleted: get_cfg(&docs,"logic_deleted").as_i64().unwrap_or_default(),
            log_dir: get_cfg(&docs,"log_dir").as_str().unwrap_or("").to_owned(),
            log_cup: get_cfg(&docs,"log_cup").as_i64().unwrap_or(0).to_owned(),
            log_temp_size: get_cfg(&docs,"log_temp_size").as_str().unwrap_or("").to_owned(),
            log_zip: get_cfg(&docs,"log_zip").as_bool().unwrap_or(false),
            log_rolling_type: get_cfg(&docs,"log_rolling_type").as_str().unwrap_or("").to_owned(),
            log_level: get_cfg(&docs,"log_level").as_str().unwrap_or("").to_owned(),
            sms_redis_send_key_prefix: get_cfg(&docs,"sms_redis_send_key_prefix").as_str().unwrap_or("").to_owned(),
            jwt_secret: get_cfg(&docs,"jwt_secret").as_str().unwrap_or("").to_owned(),
            jwt_expire: get_cfg(&docs,"jwt_expire").as_i64().unwrap_or(0).to_owned(),
            white_list_api: to_vec_string(get_cfg(&docs,"white_list_api").as_vec().unwrap().to_vec()),
            cache_type: get_cfg(&docs,"cache_type").as_str().unwrap_or("").to_owned(),
            login_fail_replay: get_cfg(&docs,"login_fail_retry").as_i64().unwrap_or(0).to_owned(),
            login_fail_retry_wait_sec: get_cfg(&docs,"login_fail_retry_wait_sec").as_i64().unwrap_or(0).to_owned(),
            super_user: to_vec_string(get_cfg(&docs,"super_user").as_vec().unwrap_or(&vec![]).to_vec()),
            user_agent_parser: get_cfg(&docs,"user_agent_parser").as_str().unwrap_or("").to_owned(),
        };
        if result.debug {
            println!("[abs_admin] debug_mode is enable!")
        }else {
            println!("[abs_admin] release_mode is enable!")
        }
        result
    }
}

// 通过配置项的 key 获取该配置项
fn get_cfg<'a>(docs: &'a Vec<Yaml>,key: &str) -> &'a Yaml{
    // 遍历 Vec<Yaml> 获取每一个 Yaml
    for x in docs {
        match x {
            Yaml::Hash(hash) =>{
                let v = hash.get(&Yaml::String(key.to_string()));
                if v.is_some() {
                    return v.unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("[abs_admin] application.yaml key: '{}' not exist",key);
}

fn to_vec_string(arg: Vec<Yaml>) -> Vec<String>{
    let mut arr = vec![];
    for x in arg {
        arr.push(x.as_str().unwrap_or("").to_string());
    }
    arr
}

// 定义全局静态变量
pub static RB: Lazy<Rbatis> = Lazy::new(||Rbatis::new());

// 初始化 rbatis
pub async fn init_rbatis() -> Rbatis{
    let config = ApplicationConfig::default();
    let mut rbatis = Rbatis::new();
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
        &config.logic_column,
        config.logic_deleted as i32,
        config.logic_un_deleted as i32,
    )));

    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#)
    }
    rbatis.link(&config.database_url).await.expect("[abs_admin] rbatis link database fail!");
    rbatis
}
