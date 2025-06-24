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

#[tokio::main]
async fn main() {
    let _config = config::get_config();
    // build our application with a single route
    let app = routes::create_router();

    println!("服务启动成功,地址：http://{}:{}", _config.host, _config.port);
    println!("服务版本为：{}", _config.version);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", _config.host, _config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
