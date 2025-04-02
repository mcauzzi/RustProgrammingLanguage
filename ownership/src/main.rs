fn main() {
    let mut s=String::from("Test");
    s.push_str("AppendedString");
    let mut clone=s.clone();
    clone.push_str("AnotherAppendedString");
    take_ownership(s);
    //println!("s is now: {s}"); // This will cause an error because s has been moved
    println!("Copied string {}",clone);

    let mut s=String::from("Hello");
    s=take_ownership_and_return(s);
    println!("s is now: {s}"); 
    println!("s's length is {}",calculate_length(&s));
    change(&mut s);
    println!("s is now: {s}");
}
fn take_ownership(s: String) {
    println!("Taking ownership of: {s}");
}
fn take_ownership_and_return(mut s: String) -> String {
    println!("Taking ownership of: {s}");
    s.push_str(" and returning it");
    s
}
fn calculate_length(s:&String)->usize{
    //s.push_str("ERROR!");
    s.len()
}
fn change(s:&mut String){
    s.push_str("Changed");
}