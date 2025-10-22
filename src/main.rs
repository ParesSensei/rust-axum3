use axum::{
    routing::{get, post, put},
    extract::{Path, Query, State, Extension},
    response::{IntoResponse, Response},
    http::StatusCode,
    Json, Router, debug_handler};
use axum_extra::TypedHeader;
use headers::{UserAgent, Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    net::{SocketAddr}};
use serde_json::json;
use sqlx::{SqlitePool, FromRow};

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
        return Err(AppError::BadRequest("Username cannot be empty".into()));
    }

    Ok(Json(UserResponse {
        message: format!("User {} success created", v_payload.username),
    }))

}

async fn show_count(State(state): State<AppState>) -> String {
    let count = *state.counter.lock().unwrap();
    format!("Count now: {}", count)
}

async fn increase(State(state): State<AppState>) -> String {
    let mut count = state.counter.lock().unwrap();
    *count += 1;
    format!("Count increase: {}", *count)
}

async fn reset_count(State(state): State<AppState>) -> Result<String, AppError> {
    let mut count = state.counter.lock().map_err(|_| AppError::Internal("failed to lock counter".into()))?;
    *count = 0;
    Ok(format!("Count now: {}", *count))
}


#[derive(Deserialize)]
struct SearchParam {
    term: String,
    page: Option<u32>,
}

async fn search(Query(param): Query<SearchParam>) -> String {
    format!("Search: {}, page: {:?}", param.term, param.page)
}

#[derive(Deserialize)]
struct LoginForm{
    username: String,
    password: String,
}
async fn login(Json(user): Json<LoginForm>) -> String {
    format!("Login.... \nuser: {}, \npassword: {}", user.username, user.password)
}


async fn handler(Extension(state): Extension<Arc<Mutex<AppState>>>) -> String {
    format!("Hello from, {}!", state.lock().unwrap().app_name)
}

async fn user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    format!("your User-agent: {}", user_agent)
}

async fn auth_header(TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> String {
    format!("Your Bearer token: {}", auth.token())
}

async fn user_book(Path((id, name, year, subject)): Path<(u32, String, u32, String)>) -> String {
    format!("User id: {}, \nname: {}, \nyear: {}, \nsubject: {}", id, name, year, subject)
}

//------------------- Database -----------------//

// #[derive(Serialize, FromRow)]
// struct AppStateDb{
//     db_pool: SqlitePool,
// }
#[derive(Clone)]
struct AppState {
    app_name: String,
    counter: Arc<Mutex<u32>>,
    db_pool: SqlitePool,
}

#[derive(Clone, FromRow, Serialize)]
struct User{
    id: u32,
    name: String,
}

async fn get_user(State(state): State<AppState>, Path(user_id): Path<u32>) -> Json<User> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    Json(user)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://users.db")
        .await
        .expect("Failed to create Sqlite database pool");

    let state = AppState {
        app_name: "Axum Project".into(),
        counter: Arc::new(Mutex::new(0)),
        db_pool: pool,
    };

    let app = Router::new()
        .route("/", get(|| async { "Halo, dunia!" }))
        .route("/hello/{name}", get(hello))
        .route("/goodbye/{name}", get(goodbye))
        .route("/congrats/{name}", get(congrats))
        .route("/users", post(create_user))
        .route("/count", get(show_count))
        .route("/count/increase", get(increase))
        .route("/count/reset", put(reset_count))
        .route("/search", get(search))
        .route("/login", post(login))
        .route("/extension", get(handler))
        .route("/ua", get(user_agent))
        .route("/auth", get(auth_header))
        .route("/path/{id}/{name}/{year}/{subject}", get(user_book))
        .route("/db/user/{id}", get(get_user))
        .with_state(state);

    println!("Server running at http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
