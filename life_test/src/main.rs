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
    let i = ImprortExcerpt {
        part: first_sentence,
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


    let amy = LolPlayer::new(String::from("amy"));
    let bob = DotaPlayer::new(String::from("bob"));
    let seven = DnfPlayer::new(String::from("seven"));

    play_game(&amy);
    play_game(&bob);
    play_game(&seven);

    let mut s = String::from("hello world");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let mut p  = Point{x: 1, y: 2};
    let r = &mut p; 
    let rr: &Point = &*r;

    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
    

}


#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

impl  Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
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
    fn share(&self) {
        println!("shared foo!")
    }
}

trait Player {
    fn play(&self);
}

struct LolPlayer {
    name: String,
}

impl LolPlayer {
    fn new(name: String) -> LolPlayer {
        LolPlayer { name }
    }
}

impl Player for LolPlayer {
    fn play(&self) {
        println!("{} play lol", self.name);
    }
}

struct DotaPlayer {
    name: String,
}

impl DotaPlayer {
    fn new(name: String) -> DotaPlayer {
        DotaPlayer { name }
    }
}

impl Player for DotaPlayer {
    fn play(&self) {
        println!("{} play dota", self.name);
    }
}

struct DnfPlayer {
    name: String,
}

impl DnfPlayer {
    fn new(name: String) -> DnfPlayer {
        DnfPlayer { name }
    }
}

impl Player for DnfPlayer {
    fn play(&self) {
        println!("{} play dnf", self.name);
    }
}

fn play_game(player: &dyn Player) {
    player.play();
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
