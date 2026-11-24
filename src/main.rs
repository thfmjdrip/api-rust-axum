use axum::{routing::get,Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_task).post(create_task));

    if let Ok(listener) = tokio::net::TcpListener::bind(
        "0.0.0.0:3001"
    ).await{ //await because it can stop for while
        println!("Listner -XD-");
        axum::serve(listener, app).await;
    }

}

async fn root() -> &'static str{
    // for auxim &'static str will return msg + 200HTTP
    "task is alive"
}

async fn health() -> &'static str{
    "ok"
}

async fn list_task() -> &'static str{
    "List of task"
}

async fn create_task() -> &'static str{
    "create_task"
}

