use std::ops::Add;

fn main() {
    let s = String::from("world");
    let s1 = string_test(&s);

    println!("{}",s);
    println!("{}",s1);

    let s2 = string_test2(s);

    println!("{}",s2);

    let mut s3 = String::from("ccc");
    
    s3.push_str("aaaaa");
    println!("{}",s3);
    let s4 = &s3;
    println!("{}\n{}",s3,s4)

}

fn string_test(s:&String) -> String{
    s.to_string()
}

fn string_test2(s:String) ->String{
    String::from("hello ").add(&s)
}
