// Becoming a Rustacian
// ---------------------

// In this exercise, you will practice various Rust programming concepts and techniques, including data types, type inference, and common idioms.

// Repeat Exercise 2: "Integer Basics" In a Rustacian way
// Repeat Exercise 3: "Float Basics" In a Rustacian way
// Repeat Exercise 4: "Boolean Basics" In a Rustacian way
// Repeat Exercise 4: "Characters Basics" In a Rustacian way


























































// Solution
// ========
// Becoming a Rustacian
// ---------------------

fn main() {
    // Exercise 1: Integer Basics
    // -------------------------

    // 1. Declare an integer variable `a` and assign the value 10 to it.
    let a = 10;

    // 2. Declare an integer variable `b` and assign the value 5 to it.
    let b = 5;

    // 3. Calculate and print the sum of `a` and `b`.
    let sum = a + b;
    println!("{}", "=".repeat(52));
    println!("Sum of a and b: {}", sum);

    // 4. Calculate and print the difference between `a` and `b`.
    let difference = a - b;
    println!("Difference between a and b: {}", difference);
    println!("{}", "=".repeat(52));

    // 5. Calculate and print the product of `a` and `b`.
    let product = a * b;
    println!("Product of a and b: {}", product);
    println!("{}", "=".repeat(52));

    // 6. Calculate and print the quotient of `a` divided by `b`.
    let quotient = a / b;
    println!("Quotient of a divided by b: {}", quotient);
    println!("{}", "=".repeat(52));

    // 7. Calculate and print the remainder of `a` divided by `b`.
    let remainder = a % b;
    println!("Remainder of a divided by b: {}", remainder);
    println!("{}", "=".repeat(52));
    // Exercise 2: Float Basics
    // -----------------------

    // 1. Declare a floating-point variable `x` and assign the value 7.5 to it.
    let x: f32 = 7.5;   // if you don't specify the type here, later you will get
                        // an error on functions that require the type explicitely like sqrt()

    // 2. Declare a floating-point variable `y` and assign the value 3.0 to it.
    let y = 3.0;

    // 3. Calculate and print the sum of `x` and `y`.
    let sum_float = x + y;
    println!("Sum of x and y: {}", sum_float);
    println!("{}", "=".repeat(52));
    // 4. Calculate and print the square root of `x`.
    let sqrt_x: f32 = x.sqrt();
    println!("Square root of x: {}", sqrt_x);
    println!("{}", "=".repeat(52));
    // Exercise 3: Boolean Basics
    // --------------------------

    // 1. Declare two boolean variables, `is_sunny` and `is_raining`, and assign values to them.
    let is_sunny = true;
    let is_raining = false;

    // 2. Calculate and print the result of "Is it sunny and raining at the same time?"
    let sunny_and_raining = is_sunny && is_raining;
    println!("Is it sunny and raining at the same time? {}", sunny_and_raining);
    println!("{}", "=".repeat(52));
    // 3. Calculate and print the result of "Is it either sunny or raining (or both)?"
    let sunny_or_raining = is_sunny || is_raining;
    println!("Is it either sunny or raining (or both)? {}", sunny_or_raining);
    println!("{}", "=".repeat(52));
    // Exercise 4: Char Basics
    // -----------------------

    // 1. Declare a character variable `my_char` and assign a character to it.
    let my_char = 'A';

    // 2. Print the character `my_char`.
    println!("Character my_char: {}", my_char);
    println!("{}", "=".repeat(52));

    // 3. Calculate and print the result of "Is `my_char` an alphabetic character?"
    let is_alphabetic = my_char.is_alphabetic();
    println!("Is my_char alphabetic? {}", is_alphabetic);
    println!("{}", "=".repeat(52));

    // 4. Declare another character variable `another_char` and assign a different character to it.
    let another_char = '5';

    // 5. Calculate and print the result of "Is `another_char` a digit?"
    let is_digit = another_char.is_digit(10); // 10 is the base for decimal digits
    println!("Is another_char a digit? {}", is_digit);
    println!("{}", "=".repeat(52));

    // 6. Calculate and print the uppercase version of `my_char`.
    let uppercase_char = my_char.to_uppercase().next().unwrap();
    
    println!("Uppercase my_char: {}", uppercase_char);
    println!("{}", "=".repeat(52));

    // 7. Calculate and print the lowercase version of `another_char`.
    let lowercase_char = another_char.to_lowercase().next().unwrap();
    
    println!("Lowercase another_char: {}", lowercase_char);
    println!("{}", "=".repeat(52));
    // Exercise 5: Type Inference and Common Idioms
    // ---------------------------------------------

    // 1. Declare a variable `i` and assign the value 2 to it without specifying the type.
    let i = 2;
    println!("{}", i);
    println!("{}", "=".repeat(52));

    // 2. Declare a variable `f` and assign the value 3.14 to it without specifying the type.
    let f = 3.14;

    println!("{}", f);
    println!("{}", "=".repeat(52));

    // 3. Create a boolean variable `is_weekend` and assign `true` to it without specifying the type.
    let is_weekend = true;

    // 4. Create a variable `greeting` and assign a string "Hello, Rust!" without specifying the type.
    let greeting = "Hello, Rust!";

    // 5. Calculate and print the result of "Is it not a weekend or not sunny?"
    let not_sunny_or_not_weekend = !is_sunny || !is_weekend;
    println!("Is it not sunny or not a weekend? {}", not_sunny_or_not_weekend);
    println!("{}", "=".repeat(52));

    // 6. Print the `greeting` string.
    println!("{}", greeting);
    println!("{}", "=".repeat(52));
}
