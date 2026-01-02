// if Expressions

fn main() {
    println!("Hello, world!");
    let num = 3;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement

    let condition = true;
    let number = if condition { 5 } else { 6 };     // Note: both arms must return same type

    println!("The value of number is: {number}");
}

// Note: In Rust, the condition in an if expression must be a boolean value.