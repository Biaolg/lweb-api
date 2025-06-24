use axum::{Router, routing::get};
use crate::controllers::user_controller;

pub fn router() -> Router {
    Router::new()
        .route("/list", get(user_controller::list_users))
}
