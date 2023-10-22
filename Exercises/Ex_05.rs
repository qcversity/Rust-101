// Exercise 05: Char Basics
// -----------------------

// In this exercise, you will work with basic character operations.

// 1. Create a Rust program that declares a character variable, `my_char`, and assigns a character to it.
// 2. Print the character `my_char`.
// 3. Use the `is_alphabetic` method to check if `my_char` is an alphabetic character, and print the result.
// 4. Create a second character variable, `another_char`, and assign a different character to it.
// 5. Use the `is_digit` method to check if `another_char` is a digit, and print the result.
// 6. Use the `to_uppercase` method to convert `my_char` to uppercase, and print the result.
// 7. Use the `to_lowercase` method to convert `another_char` to lowercase, and print the result.






















































// Solution:
// =========
fn main() {
    let my_char: char = 'a';
    let second_char: char = 'B';
    
    // Print the character
    println!("{}", "*".repeat(52));
    println!("{:^52}", "Solution");
    println!("{}", "*".repeat(52));
    println!("Character my_char: {}", my_char);

    // Check if my_char is an alphabetic character
    let is_alphabetic: bool = my_char.is_alphabetic();
    println!("Is my_char alphabetic? {}", is_alphabetic);

    let another_char: char = '5';
    
    // Check if another_char is a digit
    let is_digit: bool = another_char.is_digit(10); // 10 is the base for decimal digits
    println!("Is another_char a digit? {}", is_digit);

    // Convert my_char to uppercase
    let uppercase_char: char = my_char.to_uppercase().next().unwrap();
    println!("Uppercase my_char: {}", uppercase_char);

    // Convert another_char to lowercase
    let lowercase_char: char = second_char.to_lowercase().next().unwrap();
    println!("Lowercase another_char: {}", lowercase_char);
    println!("{}", "*".repeat(52))
}
