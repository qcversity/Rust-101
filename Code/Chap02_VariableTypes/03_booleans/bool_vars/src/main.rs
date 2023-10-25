mod practice;
use practice::practice_with_bools;

fn main() {
    make_separator();
    boolean_vars();

    make_separator();
    bool_vars_size();

    make_separator();
    convert_bool_vars();

    make_separator();
    logical_and();
    
    make_separator();
    logical_or();

    make_separator();
    logical_not();

    // Check the result on the console
    // then read the source code (writing functions will be in a separate chapter)
    make_separator();
    practice_with_bools();
    
    make_separator()
}

// A function that checks boolean variables
fn boolean_vars() {
    let is_ok: bool = true; // lowercase true
    let is_bad: bool = false; // lowercase false

    println!(
        "The boolean variables in Rust are: {} and {}",
        is_ok, is_bad
    );
}

fn bool_vars_size() {
    let x: bool = true;
    let y: bool = false;
    println!(
        "The size of {} is {} bytes.",
        x,
        std::mem::size_of::<bool>()
    );
    println!(
        "The size of {} is {} bytes.",
        y,
        std::mem::size_of::<bool>()
    );
}

// Converting Booleans
// Boolean can be converted to integers using the `as` keyword
// true will be converted to 1
// false will be converted to 0

fn convert_bool_vars() {
    let x: bool = true;
    let y: bool = false;

    let x_to_one: i32 = x as i32; // The as keyword performs the conversion
    let y_to_zero: i32 = y as i32;

    println! {"The conversion of {} is {}", x, x_to_one};
    println! {"The conversion of {} is {}", y, y_to_zero};
}

// Logical Operators:
//  1. &&: And operator
//  2. ||: Or operator

// Function to perform a logical AND operation
fn logical_and() {
    let a: bool = true;
    let b: bool = false;
    println!("{:<6} and {:<5} is ==> {}", a, a, a && a);
    println!("{:<6} and {:<5} is ==> {}", a, b, a && b);
    println!("{:<6} and {:<5} is ==> {}", b, b, b && b);
}

// Function to perform a logical OR operation
fn logical_or() {
    let a: bool = true;
    let b: bool = false;
    println!("{:<6} or {:<5} is ==> {}", a, a, a || a);
    println!("{:<6} or {:<5} is ==> {}", a, b, a || b);
    println!("{:<6} or {:<5} is ==> {}", b, b, b || b);
}

// The logical not `!`

fn logical_not() {
    let a: bool = true;
    let b: bool = false;
    println!("a is {:<6} and not a is {:<5}", a, !a);
    println!("b is {:<6} and not b is {:<5}", b, !b);
}

// Simple function for formatting the output in the console
fn make_separator() {
    println!("{}", "*".repeat(52));
}
