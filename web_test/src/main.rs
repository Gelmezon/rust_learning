use actix_web::{get,post,web,App,HttpServer,HttpResponse};

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello rust!")
}

#[post("/")]
async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}