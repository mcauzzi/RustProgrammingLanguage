fn main() {
    let s = String::from("Hello, world!");
    let index = first_word(&s);
    println!(
        "The first word is {}",
        s.get(0..index).expect("Error getting first word")
    );
    let first_word = first_word_slice(&s);
    println!("The first word with slices is {}", first_word);
    println!("the first word with a constant is {}",first_word_slice("Hello, world!"));
    //println!("s is now: {s}"); // This will cause an error because s has been moved
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
