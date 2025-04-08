fn main() {
    let number_list = vec![34, 66, 24, 27];
    let float_list = vec![34.02, 66.02323232323, 24.0123123, 27.123];
    println!("The largest number is {}", largest_in_vector(&number_list));
    println!("The largest float is {}", largest_in_vector(&float_list));
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.0, y: 10.0 };

    println!("The x coordinate of integer point is {}", integer.x());
    println!("The distance from origin of float point {float:?} is {}", float.distance_from_origin());
}

fn largest_in_vector<T: PartialOrd>(number_list: &Vec<T>) -> &T {
    let mut largest = &number_list[0];
    for number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self)-> &T{
        &self.x
    }
}

impl Point<f64>{
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}