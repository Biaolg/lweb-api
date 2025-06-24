use axum::{
    http::{Request},
    response::Response,
    middleware::Next,
    body::Body,
};
use std::time::Instant;

// 最简单的控制台日志
pub async fn console_log(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();

    println!("--> {} {}", req.method(), req.uri());

    let response = next.run(req).await;

    let duration = start.elapsed();

    println!("<-- {} ({} ms)", response.status(), duration.as_millis());

    response
}
