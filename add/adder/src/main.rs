use add_one;
use rand::Rng;
fn main() {
    let num=rand::thread_rng().r#gen();
    println!("{num}+1={}",add_one::add_one(num))
}
