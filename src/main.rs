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
    let _config = config::get_config();
    let app = routes::create_router();

    //打印基础信息
    utils::log::println_basic_info(&_config);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", _config.host, _config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
