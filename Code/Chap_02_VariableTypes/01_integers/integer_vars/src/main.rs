mod rnd_integers;

fn main() {
    // Invoke the print_signed_integers() function
    make_separator();
    print_signed_integers();
    make_separator();
    signed_integers_size();
    make_separator();

    // Invoke unsigned functions
    print_unsigned_integers();
    make_separator();
    unsigned_integers_size();
    make_separator();

    // Generate a random signed integers
    rnd_integers::print_rnd_integers()
}

// Simple function that prints `=` to pretify the console output
fn make_separator() {
    println!("{}", "=".repeat(52));
}

// a function that prints signed integers

fn print_signed_integers() {
    let i8_val: i8 = -123;
    let i16_val: i16 = -12345;
    let i32_val: i32 = -12345678;
    let i64_val: i64 = -1234567890;

    println!("i8 : {}", i8_val);
    println!("i16: {}", i16_val);
    println!("i32: {}", i32_val);
    println!("i64: {}", i64_val);
}

fn signed_integers_size() {
    let i8_val: i8 = -123;
    let i16_val: i16 = -12345;
    let i32_val: i32 = -12345678;
    let i64_val: i64 = -1234567890;

    println!("i8 : {:<12} (Size: {} bytes)", i8_val, std::mem::size_of::<i8>());
    println!("i16: {:<12} (Size: {} bytes)", i16_val, std::mem::size_of::<i16>());
    println!("i32: {:<12} (Size: {} bytes)", i32_val, std::mem::size_of::<i32>());
    println!("i64: {:<12} (Size: {} bytes)", i64_val, std::mem::size_of::<i64>());
}


// 
fn print_unsigned_integers() {
    let u8_val: u8 = 123;
    let u16_val: u16 = 12345;
    let u32_val: u32 = 12345678;
    let u64_val: u64 = 1234567890;

    println!("u8: {}", u8_val);
    println!("u16: {}", u16_val);
    println!("u32: {}", u32_val);
    println!("u64: {}", u64_val);
}

fn unsigned_integers_size() {
    let u8_val: u8 = 123;
    let u16_val: u16 = 12345;
    let u32_val: u32 = 12345678;
    let u64_val: u64 = 1234567890;

    println!("u8: {:<12} (Size: {} bytes)", u8_val, std::mem::size_of::<u8>());
    println!("u16: {:<12} (Size: {} bytes)", u16_val, std::mem::size_of::<u16>());
    println!("u32: {:<12} (Size: {} bytes)", u32_val, std::mem::size_of::<u32>());
    println!("u64: {:<12} (Size: {} bytes)", u64_val, std::mem::size_of::<u64>());
}


