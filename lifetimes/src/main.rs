use std::fmt::Display;

fn main() {
    let string1=String::from("abcd");
    let string2="xyz";

    let result=longest(string1.as_str(),string2);
    println!("The longest string is {result}");

    let novel =String::from("Call me Ishmael. Some years ago...");
    let sentence=novel.split('.').next().unwrap();
    let i= ImportantExcerpt{
        part:sentence
    };

}
fn longest<'a>(str1:&'a str,str2:&'a str)->&'a str{
    if str1.len() > str2.len(){
        str1
    }else{
        str2
    }
}

struct ImportantExcerpt<'a>{
    part:&'a str
}

impl<'a> ImportantExcerpt<'a>{
    fn level(&self)->i32{
        3
    }

    fn announce_and_return_part(&self,announcement:&str)->&str{
        println!("Attention please! {announcement}");
        self.part
    }
}

fn longest_with_an_announcement<'a,T>(x:&'a str,y:&'a str,ann:T)->&'a str where T:Display{
    println!("Announcement! {ann}");
    if x.len()>y.len(){
        x
    }else{
        y
    }
}