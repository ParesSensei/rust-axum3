use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::extract::Path;
use serde::{Deserialize, Serialize};

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


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Halo, dunia!" }))
        .route("/hello/{name}", get(hello))
        .route("/goodbye/{name}", get(goodbye))
        .route("/congrats/{name}", get(congrats));

    println!("Server berjalan di http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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