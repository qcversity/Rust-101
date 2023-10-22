// Exercise 3: Float Basics
// ------------------------

// In this exercise, you will work with basic floating-point types and perform various operations.

// 1. Create a Rust program that declares two floating-point variables, `x` and `y`, and assigns values to them.
// 2. Calculate the sum of `x` and `y`, and print the result.
// 3. Calculate the difference between `x` and `y`, and print the result.
// 4. Calculate the product of `x` and `y`, and print the result.
// 5. Calculate the quotient of `x` divided by `y`, and print the result.
// 6. Create a third floating-point variable, `z`, and assign the value 3.0 to it.
// 7. Perform the following operations and print the results:
//    - Add `x` to `z` and print the result.
//    - Subtract `y` from `z` and print the result.
//    - Multiply `x` and `z` and print the result.
//    - Divide `z` by `y` and print the result.
// 8. Calculate the square root of `x`, and print the result.
// 9. Calculate the square root of `y`, and print the result.

// Note:
//  - Choose one data type to declare your variables f32 or f64. 




















































fn main() {
    let x: f32 = 7.5;
    let y: f32 = 3.0;

    // Sum of x and y
    let sum: f32 = x + y;
    println!("Sum of x and y: {}", sum);

    // Difference between x and y
    let difference: f32 = x - y;
    println!("Difference between x and y: {}", difference);

    // Product of x and y
    let product: f32 = x * y;
    println!("Product of x and y: {}", product);

    // Quotient of x divided by y
    let quotient: f32 = x / y;
    println!("Quotient of x divided by y: {}", quotient);

    let z: f32 = 3.0;

    // Add x to z
    let add_to_z: f32 = x + z;
    println!("Add x to z: {}", add_to_z);

    // Subtract y from z
    let subtract_from_z: f32 = z - y;
    println!("Subtract y from z: {}", subtract_from_z);

    // Multiply x and z
    let multiply_x_z: f32 = x * z;
    println!("Multiply x and z: {}", multiply_x_z);

    // Divide z by y
    let divide_z_by_y: f32 = z / y;
    println!("Divide z by y: {}", divide_z_by_y);

    // Square root of x
    let sqrt_x: f32 = x.sqrt();
    println!("Square root of x: {}", sqrt_x);

    // Square root of y
    let sqrt_y: f32 = y.sqrt();
    println!("Square root of y: {}", sqrt_y);
}
