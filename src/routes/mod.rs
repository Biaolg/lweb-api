pub mod user;

use crate::middlewares::logging;
use crate::models::app::AppState;
use axum::Router;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .with_state(app_state)
        .nest("/users", user::router())
        .layer(axum::middleware::from_fn(logging::request_log))
}
