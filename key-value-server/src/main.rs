use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle));
    let listen = tokio::net::TcpListener::bind("127.0.0.1:19988").await.unwrap();
    println!("Server running on port 19988");
    axum::serve(listen, app).await.unwrap();

}


async fn handle()->Html<&'static str>{
    Html("<h1>Hello, World!</h1>")
}