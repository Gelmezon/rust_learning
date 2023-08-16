use axum::{extract::Path, response::Html, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle))
        .route("/hello/:user_name", get(hello))
        .route("/home", post(home))
        .route("/put", post(handle_put))
        .route("/get", post(handle_get));
    println!("Server running on port 19988");
    axum::Server::bind(&"0.0.0.0:19988".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn hello(Path(user_name): Path<String>) -> String {
    format!("Hello, {}!", user_name)
}

async fn home(Json(user): Json<User>) -> String {
    format!("Hello, {} year old named {}!", user.age, user.name)
}

async fn handle_put(Json(put): Json<PutCommand>) -> String {
    format!("key:{},value:{}", put.key, put.value)
}

async fn handle_get(Json(get): Json<GetCommand>) -> String {
    format!("key:{}", get.key)
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}
#[derive(Serialize, Deserialize, Debug)]
struct PutCommand {
    key: String,
    value: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GetCommand {
    key: String,
}
