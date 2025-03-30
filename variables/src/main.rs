const MINUTES_IN_A_DAY:i32=60*24;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Minutes in a day: {MINUTES_IN_A_DAY}");

    let x=5;
    let x=x+1;
    {
        let x=x*2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
