fn main() {
    variable_example();
    constant_example();
    shadow_example();
}


// VARIABLES

// by default, variables are immutable.

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// make them mutable by adding mut in front of the variable name

fn variable_example() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// CONSTANTS
// Constants aren’t just immutable by default—they’re always immutable.
// declare constants using the const keyword
// the type of the value must be annotated
// Rust’s naming convention for constants is to use all uppercase with underscores between words

fn constant_example() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;   // 10800
    println!("The maximum points are: {THREE_HOURS_IN_SECONDS}");
}



// SHADOWING
// You can also declare a new variable with the same name as a previous variable.
// The new variable shadows the previous variable.'
// This is different from marking a variable as mut because the type of the value can be changed

fn shadow_example() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}