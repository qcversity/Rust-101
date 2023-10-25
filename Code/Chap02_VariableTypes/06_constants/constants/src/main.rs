fn main() {
    make_separator();
    const_vars();

    make_separator();
    display_physical_constants();

    make_separator()
}

/* In Rust, it's a convention to use uppercase letters with underscores to name constant variables.
This convention helps distinguish constants from regular variables and makes the code more readable. While it's not a strict rule enforced by the Rust compiler, it's widely followed in the Rust community.
*/

fn const_vars() {
    // You can define compile-time constants in Rust.
    // Constants must have a specified type.
    const SECONDS_IN_HOUR: i32 = 3_600;               // The underscore is only for humans and 
                                                      // does not have any special meaning to the compiler
    const SECONDS_IN_DAY: i32 = 24 * SECONDS_IN_HOUR;
    
    // Constants are computed at compile time.
    println!("There are {} seconds in a day", SECONDS_IN_DAY);
}

// Example 
fn display_physical_constants() {

    const SPEED_OF_LIGHT: f64 = 299_792_458.0;      // meters per second
    const GRAVITY: f64 = 9.81;                      // meters per second squared
    const PLANCK_CONSTANT: f64 = 6.62607004e-34;    // joule seconds

    println!("Speed of Light: {:.2} m/s", SPEED_OF_LIGHT);
    println!("Acceleration due to Gravity: {:.2} m/s²", GRAVITY);
    println!("Planck Constant: {:10e} J·s", PLANCK_CONSTANT);
}

// Simple function for formatting the output in the console
fn make_separator() {
    println!("{}", "*".repeat(52));
}
