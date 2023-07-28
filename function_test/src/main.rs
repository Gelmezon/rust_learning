use std::ops::Add;

fn main() {
    let s = String::from("world");
    let s1 = string_test(&s);

    println!("{}", s);
    println!("{}", s1);

    let s2 = string_test2(s);

    println!("{}", s2);

    let mut s3 = String::from("ccc");

    s3.push_str("aaaaa");
    println!("{}", s3);
    let s4 = &s3;
    println!("{}\n{}", s3, s4);


    let mut l = LazyCompute::new(|v| {
        println!("{}","executing once");
        v * 3
    } );
    let r1 = l.execute(2);
    let r2 = l.execute(2);


    println!("{} {}", r1.unwrap(), r2.unwrap());


    let fn1 = |v| v * 2;
    let fn2 = |v| v * 3;
    let res = func_test(fn1, fn2)(2);
    println!("res={}", res);
}

fn string_test(s: &String) -> String {
    s.to_string()
}

fn string_test2(s: String) -> String {
    String::from("hello ").add(&s)
}

fn func_test(fn1:impl Fn(isize) -> isize , fn2:impl Fn(isize) -> isize) -> impl Fn(isize) -> isize {
    move |i | fn1(fn2(i))
}
    


struct LazyCompute {
    v: isize,
    executer: fn(isize) -> isize,
    r: Option<isize>,
}

impl LazyCompute {
    fn new(f: fn(isize) -> isize) -> LazyCompute {
        LazyCompute {
            v: 0,
            r: Option::None,
            executer: f,
        }
    }

    fn execute(&mut self, v: isize) -> Option<isize> {
        if v == self.v {
            match self.r {
                Option::Some(r) => return Option::Some(r),
                Option::None => {
                    let r = (self.executer)(v);
                    self.r = Option::Some(r);
                    return self.r;
                }
            }
        }
        self.v = v;
        let r = (self.executer)(v);
        self.r = Option::Some(r);
        return self.r;
    }
}
