fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let r = largest(&number_list);
    println!("The largest number is {}", r);
    let char_list = vec!["y", "m", "a", "q"];
    let r = largest(&char_list);
    println!("The largest char is {}", r);

   let d =  Dog::new(String::from("jerry"));
   println!("{}",d.eat());
   println!("{}",d.sleep());
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

struct Dog{
   name:String
}

impl Animal for Dog {
   fn eat(&self)->String{
      let msg = &self.name;
      return String::from(msg) + "Dog eat";
   }
   fn sleep(&self)->String{
      let msg = &self.name;
      return String::from(msg) + "Dog sleep";
   }
}

impl Dog {
   fn new(name:String)->Dog{
        Dog{name}
    }
}

pub trait Animal {
    fn eat(&self)->String;
    fn sleep(&self)->String;
}

