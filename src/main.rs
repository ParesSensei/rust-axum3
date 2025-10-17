use axum::{
    routing::{get, post, put},
    http::StatusCode,
    Json, Router,
    debug_handler
};
use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::response::{IntoResponse, Response};
use serde_json::json;

enum AppError {
    BadRequest(String),
    NotFound(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": msg})),
                ).into_response(),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": msg})),
                ).into_response(),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
                ).into_response()
        }
    }
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Halo, {}!", name)
}

async fn goodbye(Path(name): Path<String>) -> String {
    format!("Goodbye, {}!", name)
}

async fn congrats(Path(name): Path<String>) -> String {
    format!("Congrats, {}!", name)
}

#[derive(Deserialize, PartialEq)]
struct UserInput {
    username: String,
}

#[derive(Deserialize, Serialize)]
struct UserResponse {
    message: String,
}

#[debug_handler]
async fn create_user(Json(v_payload): Json<UserInput> ) -> Result<Json<UserResponse>, AppError> {
    if v_payload.username.trim().is_empty() {
        return Err(AppError::BAD_REQUEST("Username cannot be empty".into()));
    }

    Ok(Json(UserResponse {
        message: format!("User {} berhasil dibuat", v_payload.username),
    }))

}

async fn show_count(State(counter): State<Arc<Mutex<u32>>>) -> String {
    let count = *counter.lock().unwrap();
    format!("Count sekarang: {}", count)
}

async fn increase(State(counter): State<Arc<Mutex<u32>>>) -> String {
    let mut count = counter.lock().unwrap();
    *count += 1;
    format!("Count bertambah: {}", *count)
}

async fn reset_count(State(counter): State<Arc<Mutex<u32>>>) -> String {
    let mut count = counter.lock().unwrap();
    *count = 0;
    format!("Count sekarang: {}", *count)
}


#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0u32));

    let app = Router::new()
        .route("/", get(|| async { "Halo, dunia!" }))
        .route("/hello/{name}", get(hello))
        .route("/goodbye/{name}", get(goodbye))
        .route("/congrats/{name}", get(congrats))
        .route("/users", post(create_user))
        .route("/count", get(show_count))
        .route("/count/increase", get(increase))
        .route("/count/reset", put(reset_count))
        .with_state(counter);

    println!("Server berjalan di http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
