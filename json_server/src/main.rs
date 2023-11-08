use axum::{routing::post, Json, Router, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let app = Router::new().route("/json",post(json_server));

    let addr = SocketAddr::from(([0,0,0,0],19995));
    println!("Listening on {}",addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn json_server(Json(payload): Json<Payload>) -> impl IntoResponse{
    println!("payload: {:?}",payload);
    "ok"
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    contract_number: String,
    erp_contract_id: String,
}