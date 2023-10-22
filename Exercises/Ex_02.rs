// Exercise 02: Integer Basics
// -------------------------

// In this exercise, you will work with basic integer types and perform various operations.

// 1. Create a Rust program that declares two integer variables, `a` and `b`, and assigns values to them.
// 2. Calculate the sum of `a` and `b`, and print the result.
// 3. Calculate the difference between `a` and `b`, and print the result.
// 4. Calculate the product of `a` and `b`, and print the result.
// 5. Calculate the quotient of `a` divided by `b`, and print the result.
// 6. Calculate the remainder of `a` divided by `b`, and print the result.
// 7. Create a third integer variable, `c`, and assign the value 42 to it.
// 8. Perform the following operations and print the results:
//    - Add `a` to `c` and print the result.
//    - Subtract `b` from `c` and print the result.
//    - Multiply `a` and `c` and print the result.
//    - Divide `c` by `b` and print the result.








































// Solution
// ========
fn main() {
    let a: i32 = 10;
    let b: i32 = 5;

    // Sum of a and b
    let sum: i32 = a + b;
    println!("Sum of a and b: {}", sum);

    // Difference between a and b
    let difference: i32 = a - b;
    println!("Difference between a and b: {}", difference);

    // Product of a and b
    let product: i32 = a * b;
    println!("Product of a and b: {}", product);

    // Quotient of a divided by b
    let quotient: i32 = a / b;
    println!("Quotient of a divided by b: {}", quotient);

    // Remainder of a divided by b
    let remainder: i32 = a % b;
    println!("Remainder of a divided by b: {}", remainder);

    let c: i32 = 42;

    // Add a to c
    let add_to_c: i32 = a + c;
    println!("Add a to c: {}", add_to_c);

    // Subtract b from c
    let subtract_from_c: i32 = c - b;
    println!("Subtract b from c: {}", subtract_from_c);

    // Multiply a and c
    let multiply_a_c: i32 = a * c;
    println!("Multiply a and c: {}", multiply_a_c);

    // Divide c by b
    let divide_c_by_b: i32 = c / b;
    println!("Divide c by b: {}", divide_c_by_b);
}
