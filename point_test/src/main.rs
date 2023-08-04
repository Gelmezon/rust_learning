fn main() {
    let a = Box::new(3);
    println!("{}", a);

    let arr = [0;100];
    let arr1 = arr;
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0;100]);
    let arr1 = arr;
    println!("{:?}", arr1.len());
    

    let elems:Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            width: 100,
            height: 100,
            label: String::from("button1"),
        }),
        Box::new(Select {
            width: 100,
            height: 100,
            label: String::from("select2"),
        }),
    ];

    for e in elems.iter() {
        e.draw();
    }

    let arr = vec![Box::new(1), Box::new(2)];
    let (first,second) =  (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("sum = {}", sum);

    let s = gen_static_str();
    println!("{}", s);


    let x= 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

trait Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button wieht={}, height={}, label={}", self.width, self.height, self.label);
    }
}

struct Select {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Select {
    fn draw(&self) {
        println!("draw select wieht={}, height={}, label={}", self.width, self.height, self.label);
    }
}

//构造静态字符串
fn gen_static_str()-> &'static str {
    let mut s = String::new();
    s.push_str("hello world");
    Box::leak(s.into_boxed_str())
}



