use axum::{
    routing::{get, post, put},
    http::StatusCode,
    Json, Router,
    debug_handler
};
use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// #[tokio::main]
// async fn main() {
//     // initialize tracing
//     tracing_subscriber::fmt::init();
//
//     // build our application with a route
//     let app = Router::new()
//         // `GET /` goes to `root`
//         .route("/", get(root))
//         // `POST /users` goes to `create_user`
//         .route("/users", post(create_user))
//         .route("/users/{id}", get(get_user));
//
//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }
//
// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }
//
// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };
//
//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }
//
// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }
//
// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
//
//
// //-----------------Get user --------------//
// async fn get_user(Path(id): Path<u64>) -> Json<User> {
//     Json(User{
//         id,
//         username: id.to_string(),
//     })
// }



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
async fn create_user(Json(v_payload): Json<UserInput> ) -> Result<Json<UserResponse>, StatusCode> {
    if v_payload.username.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    } else {
        Ok(Json(UserResponse {
            message: format!("User {} berhasil dibuat", v_payload.username),
        }))
    }
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
