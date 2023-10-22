mod random_floats;
use random_floats::generate_random_float_f32;
use random_floats::generate_random_float_f64;

fn main() {
    make_separator();
    // invoke float_vars() function
    float_vars();
    make_separator();

    // Check the size
    float_vars_size();
    make_separator();

    // Generate random floats
    random_float_vars();
    make_separator();

    //format floats
    format_float_vars();
    make_separator();
}

fn float_vars() {
    // Rust has single-precision and double-precision floats.
    // Create an f32 float variable
    let f1: f32 = 1.23456;

    // Create an f64 float variable
    let f2: f64 = 9.87654;
    
    println!("The 32 float variable is: {}", f1);
    println!("The 64 float variable is: {}", f2);
}

// A function that checks the size of float variables

fn float_vars_size() {
    // Check the size of float variables
    let size_f32 = std::mem::size_of::<f32>();
    let size_f64 = std::mem::size_of::<f64>();

    println!("The 32 float variable size is: {} bytes", size_f32);
    println!("The 64 float variable size is: {} bytes", size_f64);
}


fn random_float_vars(){
    // Personally, I love to generate variables randomly
    // Generate random float variables

    let rnd_fl_32 = generate_random_float_f32(0, 0.0, 9.9);
    let rnd_fl_64 = generate_random_float_f64(0,0.0, 9.9);

    println!("Random float 32: {}", rnd_fl_32);
    println!("Random float 64: {}", rnd_fl_64);
    //println!("Floats to 2dp are {:.2} {:.2}", f1, f2);
}

/* --------------------------------------------------------------
     Formatting Float variables
   --------------------------------------------------------------*/
fn format_float_vars() {

    // Create two float variables
    let fl1: f32 = generate_random_float_f32(0, 1.0, 99.9);
    let fl2: f64 = generate_random_float_f64(0, 1.0, 99.9);

    println!("A 25-width float vars L-aligned filled with spaces are:\n*|{:<25}|* and\n*|{:<25}|*", fl1, fl2);
    make_separator();
    println!("A 25-width float vars R-aligned filled with space are:\n*|{:>25}|* and\n*|{:>25}|*", fl1, fl2);
    make_separator();
    println!("A 25-width float vars L-aligned filled with tilde are:\n*|{:~<25}|* and\n*|{:~<25}|*", fl1, fl2);
    make_separator();
    println!("A 25-width float vars R-aligned filled with tilde are:\n*|{:~>25}|* and\n*|{:~>25}|*", fl1, fl2);
}

// Simple function that prints `=` to pretify the console output
fn make_separator() {
    println!("{}", "=".repeat(52));
}