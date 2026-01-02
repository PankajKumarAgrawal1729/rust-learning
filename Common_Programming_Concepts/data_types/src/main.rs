// two data type subsets: scalar and compound.
// Rust is a statically typed language: which means that it must know the types of all variables at compile time.

// Scalar Types
//  four primary scalar types: integers, floating-point numbers, Booleans, and characters

// Compound Types
// Compound types can group multiple values into one type.
// Rust has two primitive compound types: tuples and arrays.

// A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// Tuples have a fixed length: Once declared, they cannot grow or shrink in size.

fn main() {
    println!("Hello, world!");
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    let five_hundred = tup.0; // accessing tuple elements directly
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of six_point_four is: {}", six_point_four);
    // The tuple without any values has a special name, unit type, and is written ().
    let unit_tuple = ((),); // tuple with one element
    println!("The value of unit_tuple is: {:?}", unit_tuple);

    // Array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first element is: {}", first);
    println!("The second element is: {}", second);

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation

    let a = [3; 5]; // all elements will be initialized to 3
    println!("The array a is: {:?}", a);
}
