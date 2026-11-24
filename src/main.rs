use axum::{
    extract::{Path,Query},
    routing::get,
    Router,
    http::StatusCode,
    Json
};

use serde::{
    Deserialize,Serialize
};

#[derive(Serialize,Clone)]
struct Task{
    id:u32,
    title:String,
    done:bool,
}
#[derive(Deserialize)]
struct CreateTask{
    title:String
}
fn seed_tasks() -> Vec<Task>{
    vec![
        Task{
            id:1,
            title:"Learn axum".to_string(),
            done:false
        },
        Task{
            id:2,
            title:"Learn axum indiano".to_string(),
            done:true
        },
        Task{
            id:3,
            title:"learn rust".to_string(),
            done:false
        },
        Task{
            id:4,
            title:"rust is awesome".to_string(),
            done:false
        },
        Task{
            id:5,
            title:"ERror".to_string(),
            done:true
        },
    ]
}

use core::task;
use std::collections::HashMap;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_task).post(create_task))
        .route("/tasks/{id}", get(get_task))
        

        ;

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

async fn list_task() -> Json<Vec<Task>>{
    Json(seed_tasks())

    
}

async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode,Json<Task>){
    let task = Task{
        id:99,
        title:payload.title,
        done:false,
    };
    (StatusCode::CREATED,Json(task))
}

async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>,StatusCode>{
    let tasks = seed_tasks();

    tasks.into_iter().find(|t|t.id == id).map(Json).ok_or(StatusCode::NOT_FOUND)

}

