use std::fmt::Display;


pub trait Summary {
    fn summarize_author(&self)->String;
    fn summarize(&self) -> String{
        format!("(Read More from {}...)",self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self)->String {
        format!("@{}",self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self)->String {
        format!("@{}",self.username)
    }
}

pub fn notify(item: &(impl Summary+Display)){
    println!("Breaking News! {}",item.summarize());
}