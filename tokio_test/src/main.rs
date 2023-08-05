use tokio::sync::mpsc as mpsc;
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        for i in 0..10 {
            let msg = say_hello(i).await;
            tx.send(msg).await.unwrap();
        }
        
    });
    while let Some(res) = rx.recv().await {
        println!("GOT = {:?}", res);
    }
}

async fn say_hello<'a>(index: usize) -> String {
    //thread sleep for 3 second
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    index.to_string()
}
