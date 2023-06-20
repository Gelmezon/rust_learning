use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main(){
    println!("web Start");
    let address = "127.0.0.1:8888";
    let listener = TcpListener::bind(address).await.expect("绑定接口失败");
    
    println!("Listening in {}",address);

    loop {
        let (mut socket,_) =  listener.accept().await.expect("接受连接失败");

        tokio::spawn(async move{
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.expect("读取数据失败");

            let req = String::from_utf8_lossy(&buffer[..n]);
            println!("req: {}",req);
            let resp = "Hello Tokio";
            socket.write_all(resp.as_bytes()).await.expect("写入数据失败");

            println!("send resp: {}",resp)
        });
    }
}
