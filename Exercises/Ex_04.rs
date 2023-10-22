// Exercise 04: Boolean Basics
// --------------------------

// In this exercise, you will work with basic boolean operations.

// 1. Create a Rust program that declares two boolean variables, `is_sunny` and `is_raining`, and assigns values to them.
// 2. Use boolean logic to calculate and print the following conditions:
//    - Is it sunny and raining at the same time?
//    - Is it either sunny or raining (or both)?
//    - Is it not sunny?
//    - Is it not raining?
// 3. Create a third boolean variable, `is_weekend`, and assign the value `true` to it.
// 4. Use boolean logic to calculate and print the following conditions:
//    - Is it a weekend and sunny at the same time?
//    - Is it a weekend or raining?
//    - Is it not a weekend?
//    - Is it not sunny or not a weekend?




















































// Solution
// --------
fn main() {
    let is_sunny: bool = true;
    let is_raining: bool = false;

    // Is it sunny and raining at the same time?
    let sunny_and_raining: bool = is_sunny && is_raining;
    println!("{}", "*".repeat(52));
    println!("{:^52}", "Solution");
    println!("{}", "*".repeat(52));
    println!("Is it sunny and raining at the same time? {}", sunny_and_raining);

    // Is it either sunny or raining (or both)?
    let sunny_or_raining: bool = is_sunny || is_raining;
    println!("Is it either sunny or raining (or both)? {}", sunny_or_raining);

    // Is it not sunny?
    let not_sunny: bool = !is_sunny;
    println!("Is it not sunny? {}", not_sunny);

    // Is it not raining?
    let not_raining: bool = !is_raining;
    println!("Is it not raining? {}", not_raining);

    let is_weekend: bool = true;

    // Is it a weekend and sunny at the same time?
    let weekend_and_sunny: bool = is_weekend && is_sunny;
    println!("Is it a weekend and sunny at the same time? {}", weekend_and_sunny);

    // Is it a weekend or raining?
    let weekend_or_raining: bool = is_weekend || is_raining;
    println!("Is it a weekend or raining? {}", weekend_or_raining);

    // Is it not a weekend?
    let not_weekend: bool = !is_weekend;
    println!("Is it not a weekend? {}", not_weekend);

    // Is it not sunny or not a weekend?
    let not_sunny_or_not_weekend: bool = !is_sunny || !is_weekend;
    println!("Is it not sunny or not a weekend? {}", not_sunny_or_not_weekend);
    println!("{}", "*".repeat(52));
}
