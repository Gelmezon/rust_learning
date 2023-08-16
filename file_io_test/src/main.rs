use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()>{
    let mut file = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    let r = file.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", r);
    println!("The bytes: {:?}", buffer);
    let msg = String::from_utf8_lossy(&buffer);
    println!("The msg: {:?}", msg);


    let mut file_two = File::create("foo_two.txt").await?;
    file_two.write_all(b"some bytes").await?;

    println!("wrote to foo_two.txt");
    Ok(())
}