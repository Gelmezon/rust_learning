fn main() {
    let x = 5;
    let y = x;
    println!("{}", x);

    let x = "abcd";
    let y = x;
    println!("{}", x);

    let x = String::from("aaaaa");
    let y = &x;
    let z = x.clone();
    println!("{}", z);

    let mut x = String::from("aaaaa");
    let y = &mut x;
    let z = calculate_length(y);

    println!("{}", z);

    let x = "pick";
    greet(String::from(x));
    let y = String::from(x);
    greet_str(&y[..]);

    let x = File {
        name: String::from("a.txt"),
        data: vec![1, 2, 3, 4, 5],
    };

    let x_name = &x.name;
    let x_data = &x.data.len();
    println!("{:?}",x);
    println!("{} is {} bytes long", x_name, x_data);
    
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn greet_str(name: &str) {
    println!("Hello, {}!", name);
}

/**
 * 1. 借用
 */
fn _borrow(s: &String) -> &String {
    s
}

/**
 * 1. 获取字符串中第一个空格的位置
 */
fn _get_space(s: &str) -> (usize, &str) {
    let l = s.as_bytes();
    for (i, &item) in l.iter().enumerate() {
        if item == b' ' {
            return (i, &s[0..i]);
        }
    }
    return (s.len(), &s[..]);
}

struct People {
    age: usize,
}

impl Clone for People {
    fn clone(&self) -> Self {
        People { age: self.age }
    }
}

impl Copy for People {}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("aaaa");
    s.len()
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}
