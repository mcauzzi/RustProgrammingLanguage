#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_result()->Result<(),String>{
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("Expected 4, got {}", result))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle {
            width: 10,
            height: 10,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let rect1 = Rectangle {
            width: 5,
            height: 5,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 10,
        };
        assert!(!rect1.can_hold(&rect2));
    }
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greeting did not contain name. Value was `{}`", result);
    }
    #[test]
    #[should_panic(expected = "Guess value must be less than 100, got 200")]
    fn guess_greater_than_100() {
        Guess::new(200);
    }
}
pub struct Guess {
    value: u32,
}
impl Guess{
    pub fn new(value:u32)->Guess{
        if value<1{
            panic!("Guess value must be greater than 0, got {}",value);
        }else if value >100{
            panic!("Guess value must be less than 100, got {}",value);
        }
        Guess{value}
    }
    pub fn value(&self)->u32{
        self.value
    }
}
pub fn greeting(name: &str)->String{
    format!("Hello {name}!")
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn add_two(a: u64) -> u64 {
    a + 2
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
