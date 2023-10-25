
// Function to check if a number is even

pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}

// Function to check if a number is prime
pub fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..(number / 2 + 1) {
        if number % i == 0 {
            return false;
        }
    }
    true
}
// Logical operators

// Function to perform a logical AND operation
fn logical_and(a: bool, b: bool) -> bool {
    a && b
}

// Function to perform a logical OR operation
fn logical_or(a: bool, b: bool) -> bool {
    a || b
}
pub fn practice_with_bools() {
    let numbers = 1..=7; // Create a range of values from 1 to 9 (inclusive).

    for number in numbers {
        let result = is_even(number);
        println!("Is {} even? {}", number, result);
    }
    println!("{}", "*".repeat(52));
    let numbers_2 = (1..=17).step_by(2);
    for number in numbers_2 {
        let is_prime_num = is_prime(number);
        println!("Is {} a prime number? {}", number, is_prime_num);
    }
    println!("{}", "*".repeat(52));
    let a = true;
    let b = false;
    let and_result = logical_and(a, b);
    println!("Logical AND: {:<10} && {:<10} = {}", a, b, and_result);

    let or_result = logical_or(a, b);
    println!("Logical OR : {:<10} || {:<10} = {}", a, b, or_result);
}

