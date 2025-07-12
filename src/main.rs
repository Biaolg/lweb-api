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
    //打印基础信息
    utils::log::println_basic_info(&config);

    let db_map = database::init_database(&config).await.unwrap();

    let app_state = models::app::AppState { db_map };

    let app = routes::create_router(app_state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
