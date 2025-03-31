fn main() {
    let num = 6;

    if_multiple_branches(num);
    if_assignment();
    loop_with_break();

    nested_loop_with_break();

    countdown();

    let a = [10, 20, 30, 40, 50];

    iterate_array_while(a);

    iterate_array_for(a);

    fizz_buzz();
}

fn fizz_buzz() {
    for number in 1..100 {
        if number % 3 == 0 {
            if number % 5 == 0 {
                println!("FizzBuzz");
            } else {
                println!("Fizz");
            }
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{number}");
        }
    }
}

fn iterate_array_for(a: [i32; 5]) {
    for element in a {
        println!("The value is: {element}");
    }
}

fn iterate_array_while(a: [i32; 5]) {
    let mut index = 0;
    while index < a.len() {
        println!("The value at index {index} is: {}", a[index]);
        index += 1;
    }
}

fn countdown() {
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF!!!");
}

fn nested_loop_with_break() {
    let mut count = 0;
    'counting_up: loop {
        println!("count={count}");
        let mut remaining = 10;
        loop {
            println!("remaining={remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            if count == 3 {
                break;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count {count}");
}

fn loop_with_break() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");
}

fn if_assignment() {
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {num}");
}

fn if_multiple_branches(num: i32) {
    if num % 4 == 0 {
        println!("The number is divisible by 4");
    } else if num % 3 == 0 {
        println!("The number is divisible by 3");
    } else if num % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 2,3 or 4");
    }
}
