use std::thread;

fn main() {
    let store=Inventory{
        shirts:vec![ ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue]
    };
    let user_pref1=Some(ShirtColor::Red);

    let giveaway1=store.give_away(user_pref1);
    println!("The user  with preference {:?} gets a shirt of color {:?}",user_pref1,giveaway1);

    let user_pref2=None;
    let giveaway2=store.give_away(user_pref2);
    println!("The user with preference {:?} gets a shirt of color {:?}",user_pref2,giveaway2);

    let list=vec![1,2,3];

    let only_borrows=|| println!("From closure {:?}",list);
    println!("Before closure: {:?}",list);
    only_borrows();
    println!("After closure: {:?}",list);
    let mut list=vec![1,2,3];
    println!("Before closure: {:?}",list);
    let mut borrows_mutably=|| list.push(8);

    borrows_mutably();
    println!("After closure {:?}",list);

    thread::spawn(move || {
        println!("From thread {:?}",list);
    }).join().unwrap();
    let mut list=[Rectangle{width: 10,height: 20},Rectangle{width: 30,height: 40},Rectangle{width: 50,height: 60}];
    list.sort_by_key(|x|x.width);
    println!("Sorted by width {:#?}",list);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
