use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::server::conn::http1;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};
use hyper::service::service_fn;
use tokio::net::TcpListener;
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let resp = Response::builder()
        .header("Content-type", "text/plain")
        .body(Full::new(Bytes::from("HELLO WORLD")))
        .unwrap();
    Ok(resp)
}
