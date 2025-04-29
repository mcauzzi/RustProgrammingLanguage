#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    RcCons(i32,Rc<List>),
    Nil,
}
use std::rc::Rc;

use BoxT::{custom_smart_pointer::CustomSmartPointer, my_box::MyBox};

use crate::List::{Cons, RcCons,Nil};
fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Cons value= {:?}", list);
    let x = 5;
    let y = Box::new(x);
    let my_y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *my_y);

    let name=MyBox::new(String::from("Rust"));
    hello(&name);
    let c=CustomSmartPointer {
        data:String::from("Stuff")
    };
    drop(c);
    let d=CustomSmartPointer {
        data:String::from("More Stuff")
    };
    println!("CustomSmartPointer created");

    let a=Rc::new(RcCons(5,Rc::new(RcCons(10,Rc::new(Nil)))));
    println!("Count at the start = {}", Rc::strong_count(&a));
    let b=RcCons(3,Rc::clone(&a));
    println!("Count after b created = {}", Rc::strong_count(&a));
    {
        let d=RcCons(4,Rc::clone(&a));
        println!("Count after d created = {}", Rc::strong_count(&a));
    }
    println!("Count after d goes out of scope = {}", Rc::strong_count(&a));
    let c=RcCons(4,Rc::clone(&a));
    println!("Count after c created = {}", Rc::strong_count(&a));
}

fn hello(name:&str){
    println!("Hello {name}")
}