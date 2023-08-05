use std::thread;
fn main() {
    let name = String::from("thread-001");
    let thread = thread::Builder::new()
        .name(name.clone())
        .spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                for i in 0..10 {
                    let current_name = name.clone();
                    tokio::spawn(async move {
                        let msg = say_hello(i).await;
                        println!("{} GOT = {:?}", current_name, msg);
                    });
                }
            });
        })
        .unwrap();
    thread.join().unwrap();
    loop {}
}
 
async fn say_hello(index: usize) -> String {
    //thread block for 3 second
    thread::sleep(std::time::Duration::from_secs(3));
    index.to_string()
}