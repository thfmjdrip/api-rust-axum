use axum::{routing::get,Router};

use crate::handlers::task;
use crate::state::AppState;

pub fn app(state:AppState) -> Router{
    Router::new()
    .route("/healt", get(health))
    .route("/tasks", get(task::list).post(task::create))
    .route(
        "/tasks/{id}",
        get(task::get_one).delete(task::delete).patch(task::update))
    .with_state(state)

}


async fn health() -> &'static str{
    "OK"
}
