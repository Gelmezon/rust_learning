use life_test::ImprortExcerpt;
use life_test::NewArticle;
use life_test::Summary;
use life_test::Tweet;
fn main() {

    let s1 = String::from("hellow");
    let s3: &String;
    let s2 = String::from("11111");
    s3 = max(&s1, &s2);
    println!("{}", s3);

    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("super cool!!!!!"),
        reply: false,
        retweet: false,
    };
    let msg = tweet.summarize();
    println!("{}", msg);

    let new_article = NewArticle {
        headline: String::from("headline"),
        location: String::from("shenzhen"),
        autor: String::from("Feitan"),
        content: String::from("sshdfaskhdasydiwqda"),
    };
    let msg = new_article.summarize();
    println!("{}", msg);

    let array = vec![String::from("1"), String::from("2"), String::from("3")];
    for item in array.iter() {
        println!("{}", item)
    }

    let novel = String::from("call me ishmael  .some years age ...");
    let first_sentence = novel.split(".").next().expect("could not found a '.'");
    let i = ImprortExcerpt{
        part:first_sentence
    };
    i.show();


    let mut data = vec![1, 2, 3];

    // 创建一个不可变引用
    let reference1 = &data;

    println!("reference1: {:?}", reference1);

    // 在不可变引用的生命周期结束后，创建一个可变引用
    let reference2 = &mut data;

    // 可以在此处通过可变引用修改 data
    reference2.push(4);

    // 输出 data
    println!("data: {:?}", reference2);


    let mut foo = Foo;

    let b = &mut foo;
    b.share();
    println!("{:?}", foo);

    
}

fn max<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    }
    s2
}


#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {
        println!("shared foo!")
    }
}


// fn get_default<'m,K,V>(map: &'m mut std::collections::HashMap<K,V>, key: K) -> &'m mut V
// where K: std::cmp::Eq + std::hash::Hash+Clone,
//       V: Default
// {
//     match map.get_mut(&key) {
//         Some(v) => v,
//         None => {
//             map.insert(key.clone(), V::default());
//             map.get_mut(&key).unwrap()
//         }
//     }
// }


    


