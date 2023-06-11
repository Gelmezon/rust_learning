fn main() {
    let s = Coin::One(20);
    let s2 = Coin::Five(10);
    let s3 = Coin::Ten(5);
    let s4 = Coin::Hundred(1);
    println!("value: {}", (s.value()+s2.value()+s3.value()+s4.value()));


    let w = Some(5);
    let a = w.unwrap();
    println!("a: {}", a);


    let n : Option<i32> =None;
    let n1 = n.and(Some(1));
    let res = n1.unwrap_or(0);
    println!("res: {}", res);
}

/**
 * 枚举
 */
enum Coin{
    One(usize),
    Five(usize),
    Ten(usize),
    Hundred(usize),
}

/**
 * 枚举的方法
 */
impl Coin{
    /**
     * 計算硬幣的價值
     */
    fn value(&self) -> usize{
        match self{
            Coin::One(v) => v * 1,
            Coin::Five(v) => v * 5,
            Coin::Ten(v) => v * 10,
            Coin::Hundred(v) => v * 100,
        }
    }
}
