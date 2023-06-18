use life_test::Tweet;
use life_test::Summary;
use life_test::NewArticle;
fn main() {
    let s1 = String::from("hellow");
    let s3: &String;
    let s2 = String::from("11111");
    s3 = max(&s1, &s2);
    println!("{}", s3);


    let tweet = Tweet{
        username:String::from("horse_ebook"),
        content:String::from("super cool!!!!!"),
        reply:false,
        retweet:false,
    };
    let msg = tweet.summarize();
    println!("{}",msg);

    let new_article = NewArticle{
        headline:String::from("headline"),
        location:String::from("shenzhen"),
        autor:String::from("Feitan"),
        content:String::from("sshdfaskhdasydiwqda"),
    };
    let msg = new_article.summarize();
    println!("{}",msg);
}

fn max<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}
