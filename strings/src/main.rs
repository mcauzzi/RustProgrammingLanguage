fn main() {
    let mut s=String::from("foo");
    s.push_str(" bar");

    let s1=String::from("Hello, ");
    let s2=String::from("world!");
    let s3=s1+&s2;

    let tic=String::from("tic");
    let tac= String::from("tac");
    let toe=String::from("toe");
    let s=format!("{tic}-{tac}-{toe}");
    for c in s.chars(){
        println!("{c}");
    }
}
