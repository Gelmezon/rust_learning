use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello rust!")
}

#[post("/")]
async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

#[get("/kokia")]
async fn kokia() -> HttpResponse {
    HttpResponse::Ok().body("Kokia")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(kokia)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 19000))?
    .run()
    .await
}