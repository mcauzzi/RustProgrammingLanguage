use traits::aggregator::{NewsArticle, Summary, Tweet};
fn main() {
    let tweet= Tweet{
        content: String::from("Test tweet"),
        reply:false,
        retweet:false,
        username:String::from("horse_ebooks")
    };

    println!("1 new tweet {}",tweet.summarize());

    let news_article=NewsArticle{
        author:String::from("Test Author"),
        headline: String::from("TEST_HEADLINE"),
        location: String::from("Mantua"),
        content:String::from_utf8(vec![b'A';10]).unwrap()
    };
    println!("1 new news article {}, content {}",news_article.summarize(),news_article.content);
}

