use jsonc_parser::parse_to_serde_value;
use std::env;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::log_error;
use crate::models::app::Config;

fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let value = parse_to_serde_value(&content, &Default::default())?.unwrap(); // 解析为 serde_json::Value
    let config: Config = serde_json::from_value(value)?;
    Ok(config)
}

fn get_named_arg(key: &str) -> Option<String> {
    let prefix = format!("{}=", key);
    for arg in env::args().skip(1) {
        if let Some(val) = arg.strip_prefix(&prefix) {
            return Some(val.to_string());
        }
    }
    None
}

pub fn get_config() -> Config {
    let env_value = get_named_arg("env").unwrap_or_else(|| "app".to_string());

    let rel_path = match env_value.as_str() {
        "dev" => "./config/dev.config.jsonc",
        "app" | "prod" | _ => "./config/app.config.jsonc",
    };

    // 将相对路径转为绝对路径（如果失败就 fallback 用原路径）
    let abs_path: PathBuf = fs::canonicalize(rel_path).unwrap_or_else(|err| {
        log_error!("警告：无法转换为绝对路径，原因：{}\n使用相对路径继续：{}", err, rel_path);
        PathBuf::from(rel_path)
    });

    load_config(&abs_path).unwrap_or_else(|err| {
        log_error!("配置加载失败: {}", err);
        log_error!("尝试加载的路径: {}", abs_path.display());
        std::process::exit(1);
    })
}
