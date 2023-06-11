fn main() {
    let s = String::from("hello world");
    let s1 = borrow(&s);
    println!("{}",s);
    println!("{}",s1);
    let (index,res) = get_space(&s1[..]);
    print!("{},{}",index,res);
}

/**
 * 1. 借用
 */
fn borrow(s: &String)->&String{
    s
}

/**
 * 1. 获取字符串中第一个空格的位置
 */
fn get_space(s: &str) -> (usize, &str){
    let l = s.as_bytes();
    for (i,&item) in l.iter().enumerate(){
        if item == b' '{
            return (i,&s[0..i]);
        }
    }
    return  (s.len(), &s[..])
}
