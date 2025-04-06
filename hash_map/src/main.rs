 use std::collections::HashMap;
fn main() {
    let mut scores=HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 20);

    let score=scores.get("Blue").copied().unwrap_or(0);
    println!("The score is {score}");

    for (k,v) in &scores{
        println!("The score for team {k} is {v}");
    }

    scores.entry(String::from("Blue")).or_insert(40);

    println!("{scores:?}");

    let text="hello world wonderful world";

    let mut map=HashMap::new();

    for str in text.split_whitespace(){
        let count=map.entry(str).or_insert(0);
        *count+=1;
    }
    println!("{map:?}")

}
