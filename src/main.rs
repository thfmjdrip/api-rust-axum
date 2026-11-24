
mod models;
mod state;
mod handlers;
mod route;




use core::task;
use std::collections::HashMap;

use crate::state::AppState;
#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = route::app(state);

    if let Ok(listener) = tokio::net::TcpListener::bind(
        "0.0.0.0:3001"
    ).await{ //await because it can stop for while
        println!("Listner -XD-");
        axum::serve(listener, app).await;
    }
    
}

