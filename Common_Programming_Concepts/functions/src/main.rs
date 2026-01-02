// Statements and Expressions
// In Rust, statements are instructions that perform some action and do not return a value. 
// Expressions, on the other hand, evaluate to a value. For example, let x = 5; is a statement, while 5 + 1 is an expression that evaluates to 6.
// Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression

fn main() {
    println!("Hello, world!");
    another_function();
    print_age(26);

    let y = {
        let x = 3;
        x + 1          // Expressions do not include ending semicolons, so this line returns the value 4
    };

    println!("The value of y is: {y}");
    let num = five();
    println!("The value returned from five() is: {num}");
    let ans = plus_one(5);
    println!("The value returned from plus_one(5) is: {ans}");
}

fn another_function() {
    println!("Another Function");
}

fn print_age(age: i32) {
    println!("your age is {age}");
}

// Functions with Return Values

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}