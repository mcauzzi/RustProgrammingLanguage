enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState{
    Alabama, Alaska,
}
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let six=plus_one(Some(5));
    println!("5 + 1 = {}",match six {
        None=>0,
        Some(val)=>val
    });

    let dice_roll=8;
    match dice_roll{
        3=>add_hat(),
        7=>remove_hat(),
        _=>reroll(),
    }
}
fn add_hat(){}
fn remove_hat(){}
fn reroll(){}
fn value_in_cents(coin: Coin)->u8{
    match coin{
        Coin::Dime=>10,
        Coin::Nickel=>5,
        Coin::Penny=>1,
        Coin::Quarter(state)=>{
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None=>None,
        Some(x)=>Some(x+1)
    }
}
