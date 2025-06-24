use axum::response::Json;
use serde_json::json;

use crate::services::user_service;

pub async fn list_users() -> Json<serde_json::Value> {
    let users = user_service::get_all_users().await;
    Json(json!({
        "code": 0,
        "data": users
    }))
}
