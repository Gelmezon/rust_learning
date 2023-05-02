use std::ops::Add;


fn main() {
    let s = String::from("world");
    let s1 = string_test(&s);

    println!("{}",s);
    println!("{}",s1);

    let s2 = string_test2(s);

    println!("{}",s2);

}

fn string_test(s:&String) -> String{
    s.to_string()
}

fn string_test2(s:String) ->String{
    String::from("hello ").add(&s)
}
