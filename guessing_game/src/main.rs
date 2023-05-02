use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("欢迎游戏！！");
    println!("请输入数字");
    let random = rand::thread_rng().gen_range(1, 100);
    let  a:[i32;5] = [1000_000,2,3,4,5];
    for ele in a {
        println!("{}",ele)
    }
    let index = 15;
    let r = a[index];
    println!("{}",r);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("输入数字有误"); 
        let value:u32 = guess.trim().parse().expect("数字转换错误");
        match value.cmp(&random) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                print!("猜对了");
                return;
            } 
        }
    }
    
}

