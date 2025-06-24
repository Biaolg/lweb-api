pub mod user;

use axum::Router;
use crate::middlewares::logging;

pub fn create_router() -> Router {
    Router::new()
        .nest("/users", user::router())
        .layer(axum::middleware::from_fn(logging::console_log))
}
