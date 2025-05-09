#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
impl Rectangle{
    fn area(&self)->u32{
        self.height*self.width
    }
    fn can_hold(&self, other:&Rectangle)->bool{
        self.height>other.height && self.width>other.width
    }

    fn square(size:u32)->Self{
        Self { width: size, height: size }
    }
}
fn main() {
    let rect=Rectangle{width:64,height:32};
    println!("The area of the rectangle is {}",rect.area());
    println!("Rect is {rect:#?}");
    let rect1=Rectangle{
        width:30,
        height:50
    };
    let rect2=Rectangle{
        width:10,
        height:40
    };
    let rect3=Rectangle{
        width:60,
        height:45
    };
    println!("Can rect1 hold rect2?: {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}",rect1.can_hold(&rect3));
    let square=Rectangle::square(42);
    println!("Square is {square:#?}")
}
