use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
#[tokio::main]
async fn main() {
    let listen = TcpListener::bind("10.0.2.224:19001").await.unwrap();
    loop {
        let (socket, address) = listen.accept().await.unwrap();
        println!("accept from: {}", address);
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let resp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),   
        };

        connection.write_frame(&resp).await.unwrap();
    }
    
}
