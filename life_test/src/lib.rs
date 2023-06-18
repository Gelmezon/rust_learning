pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle{
    pub headline:String,
    pub location:String,
    pub autor:String,
    pub content:String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}({})",self.headline,self.autor,self.location)
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",self.username,self.content)
    }
}