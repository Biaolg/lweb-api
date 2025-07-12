// 依赖
use axum;

//框架
mod controllers;
mod database;
mod middlewares;
mod routes;
mod services;
// 模型
mod models;
// 配置
mod config;
//工具
mod utils;

#[tokio::main]
async fn main() {
    let config = config::get_config();
    let app = routes::create_router();
    
    //打印基础信息
    utils::log::println_basic_info(&config);

    log_info!("数据库{}", config.databases.len());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
