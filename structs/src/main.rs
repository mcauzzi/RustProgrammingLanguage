struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
fn main() {
    basic_structs();
    tuple_structs();
}

fn tuple_structs() {
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
    let Point(x,y,z)=origin;
}

fn basic_structs() {
    let mut admin_user = User {
        username: String::from("admin"),
        email: String::from("admin@gmail.com"),
        active: false,
        sign_in_count: 0,
    };
    admin_user.email=String::from("newadmin@gmail.com");
    let test_user=build_user("test", "test@gmail.com");
    println!("User1's username is {}",test_user.username);
    let test2_user= User {
        username:String::from("test2"),
        ..test_user
    };
    println!("{}",test_user.username);
}

fn build_user(username:&str,email:&str)->User{
    User { active: true, username: String::from(username), email: String::from(email), sign_in_count: 0 }
}