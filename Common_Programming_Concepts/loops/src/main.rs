// Rust has three kinds of loops: loop, while, and for

// 1. Repeating Code with loop
// The loop keyword creates an infinite loop that can be exited with a break statement.
fn loop_example() {
    // loop {
    //     println!("again!");    // This will print "again!" indefinitely
    // }

    // Returning Values from Loops
    // Nested loops with labels allow you to specify which loop to break out of.
    // In this example, when temp reaches 2, the outer loop labeled 'inc_temp is exited.
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count; // Exit the loop when count reaches 5
        }
        println!("count = {}", count);
    };
    println!("The result is {}", result); // This will print "The result is 5"

    // loop labels
    let mut temp: i32 = 0;

    'inc_temp: loop {
        println!("temp: {temp}");
        let mut remain: i32 = 0;
        loop {
            if temp == 2 {
                break 'inc_temp;
            }
            if remain == 5 {
                break;
            }
            println!("remain: {remain}");
            remain += 1;
        }
        temp += 1;
    }

    // While Loops
    let mut num = 3;

    while num != 0 {
        println!("num: {num}");
        num -= 1;
    }
    
    let a = [1, 2, 3];
    let mut idx = 0;

    while idx < 3 {
        println!("{0}", a[idx]);
        idx += 1;
    }

    // For Loop

    for ele in a {
        println!("The value is {ele}");
    }

    // For loop with range

    for num in 1..4 {
        println!("Number : {num}");
    }

    // range in reverse
    for num in (1..4).rev() {
        println!("Reverse Number: {num}");
    }
}

fn main() {
    println!("Hello, world!");
    loop_example();
}
