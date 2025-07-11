use axum::{body::Body, http::Request, middleware::Next, response::Response};
use colored::*;
use std::time::Instant;

// 最简单的控制台日志
pub async fn console_log(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();

    let method_type = if req.method() == "GET" {
        " GET ".on_green()
    } else if req.method() == "POST" {
        " POST ".on_yellow()
    } else if req.method() == "PUT" {
        " PUT ".on_blue()
    } else if req.method() == "DELETE" {
        " DELETE ".on_red()
    } else {
        req.method().to_string().on_white()
    };

    crate::logline!(
        "--> {} {}",
        method_type,
        req.uri().to_string().bright_green().underline()
    );

    let response = next.run(req).await;

    let duration = start.elapsed();

    let status = response.status().as_u16();

    let status_code = if status >= 200 && status < 300 {
        format!(" {} ", status).on_green()
    } else if status >= 300 && status < 400 {
        format!(" {} ", status).on_yellow()
    } else if status >= 400 && status < 500 {
        format!(" {} ", status).on_red()
    } else {
        format!(" {} ", status).on_white()
    };

    let duration_color_str = if duration.as_millis() <= 500 {
        format!(" {} ", duration.as_millis()).green()
    }else if duration.as_millis() <= 1000 {
        format!(" {} ", duration.as_millis()).blue()
    }else if duration.as_millis() <= 10 * 1000 {
        format!(" {} ", duration.as_millis()).yellow()
    }else{
        format!(" {} ", duration.as_millis()).on_red()
    };

    crate::logline!("<-- {} ({} ms)", status_code, duration_color_str);
    println!("");

    response
}
