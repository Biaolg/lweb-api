use crate::models::config;
use colored::*;
#[macro_export]
macro_rules! logline {
    ($($arg:tt)*) => {{
        // 使用相对路径（不是仅文件名）
        let file = std::path::Path::new(file!())
            .to_str()
            .unwrap();
        let line = line!();
        let location = format!("[{}:{}]", file, line).dimmed();

        println!("{} {}", location, format!($($arg)*));
    }};
}


//打印基础信息
pub fn println_basic_info(_config: &config::Config) {
    println!("");
    logline!("{}"," ----------------------------LWEB-API---------------------------- ".on_blue().bold());
    logline!("{}{}","版本：".dimmed(), _config.version.cyan());
    logline!("{}{}","地址：".dimmed(), format!("http://{}:{}", _config.host, _config.port).underline().bright_green());
    logline!("{}"," ---------------------------------------------------------------- ".on_blue().bold());
    println!("");
}
