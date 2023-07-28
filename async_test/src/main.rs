use futures::executor::block_on;
fn main() {

    let future = do_something();
    block_on(future);

    let future = hello_world();
    block_on(future);

    block_on(async_main());
}

async fn async_main() {

    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

async fn do_something(){
    println!("do something");
}


async fn hello_world(){
    hello_cat().await;
    println!("hello world")
}

async fn hello_cat(){
    println!("hello cat")
}

struct Song{
    author: String,
    name: String,
}

async fn learn_song() -> Song{
    Song{
        author: String::from("kokia"),
        name: String::from("little song"),
    }
}

async fn sing_song(song: &Song){
    println!("sing song: {} - {}", song.author, song.name);
}

async fn dance(){
    println!("dance");
}

async fn learn_and_sing(){
    let song = learn_song().await;
    //sleep 1秒
    std::thread::sleep(std::time::Duration::from_secs(1));
    sing_song(&song).await;
}

