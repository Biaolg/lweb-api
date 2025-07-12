use crate::models::app_config;

use colored::*;
use once_cell::sync::Lazy;
use std::fs::OpenOptions;
use std::sync::Mutex;

pub static LOG_FILE: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
    std::fs::create_dir_all("logs").ok(); // 确保 logs 目录存在
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/app.log")
        .expect("Cannot open log file");
    Mutex::new(file)
});

// 日志宏 基础
#[macro_export]
macro_rules! logline {
    ($($arg:tt)*) => {{
        use colored::*;
        use chrono::Local;
        use $crate::utils::log::LOG_FILE;
        use std::io::Write;

        let file = std::path::Path::new(file!())
            .to_str()
            .unwrap();
        let line = line!();
        let location = format!("[{}:{}]", file, line).dimmed();

        let msg_colored = format!("{}", format!($($arg)*));
        let console_output = format!("{} {}", location, msg_colored);

        // 打印到控制台（有颜色）
        println!("{}", console_output);

        // 写入日志文件（去除颜色）
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let stripped = strip_ansi_escapes::strip(console_output.as_bytes())
            .unwrap_or_else(|_| console_output.clone().into_bytes());
        let log_line = format!("[{}] {}\n", now, String::from_utf8_lossy(&stripped));

        if let Ok(mut file) = LOG_FILE.lock() {
            let _ = file.write_all(log_line.as_bytes());
        }
    }};
}

// 日志宏 信息
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        // INFO 白色背景
        crate::logline!("{} {}", "[INFO]".blue(), format!($($arg)*))
    };
}

// 日志宏 警告
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        crate::logline!("{} {}", "[WARN]".on_yellow(), format!($($arg)*).yellow())
    };
}

// 日志宏 错误
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        crate::logline!("{} {}", "[ERROR]".on_red(), format!($($arg)*).bright_red())
    };
}

//打印基础信息
pub fn println_basic_info(_config: &app_config::Config) {
    println!("");
    logline!(
        "{}",
        " ----------------------------LWEB-API---------------------------- "
            .on_blue()
            .bold()
    );
    logline!("{}{}", "版本：".dimmed(), _config.version.cyan());
    logline!(
        "{}{}",
        "地址：".dimmed(),
        format!("http://{}:{}", _config.host, _config.port)
            .underline()
            .bright_green()
    );
    logline!(
        "{}",
        " ---------------------------------------------------------------- "
            .on_blue()
            .bold()
    );
    println!("");
}
