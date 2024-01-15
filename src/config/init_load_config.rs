use schemars::schema::RootSchema;
use serde::de::DeserializeOwned;
use crate::config::config::{EnvConfig, GlobalConfig};
use tracing::info;

fn load_env_config() -> Option<EnvConfig> {
    load_config::<EnvConfig>("application.yml")
}

fn load_global_config_from_env(active: String) -> Option<GlobalConfig> {
    let path = format!("application-{}.yml", active);
    load_config::<GlobalConfig>(&path)
}

pub fn load_global_config() -> Option<GlobalConfig> {
    if let Some(env_config) = load_env_config() {
        return load_global_config_from_env(env_config.profiles.active);
    }
    None
}

fn load_config<T>(path: &str) -> Option<T> where T: DeserializeOwned {
    // 1.通过std::fs读取配置文件内容
    // 2.通过serde_yaml解析读取到的yaml配置转换成json对象
    match serde_yaml::from_str::<RootSchema>(&std::fs::read_to_string(path).expect(&format!("failure read file {}", path))) {
        Ok(root_schema) => {
            // 通过serde_json把json对象转换指定的model
            let data = serde_json::to_string_pretty(&root_schema).expect("");
            let config = serde_json::from_str::<T>(&*data).expect("");
            // 返回格式化结果
            Some(config)
        }
        Err(err) => {
            info!("{}",err);
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::init_load_config::load_global_config;
    // use crate::config::config::GlobalConfig;

    #[test]
    pub fn load_config_test() {
        match load_global_config() {
            None => {
                println!("None");
            }
            Some(config) => {
                println!("{:#?}", config);
            }
        }
    }
}