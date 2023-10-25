mod practice;
use practice::is_vowel;
use practice::char_to_ascii;
use practice::is_digit_or_letter;

fn main() {
    make_separator();
    char_vars();

    make_separator();
    char_vars_size();

    make_separator();
    convert_char_vars();

    // practice functions
    // You are not expected to understand the source code of these functions. 
    make_separator();
    is_vowel();

    make_separator();
    char_to_ascii();

    make_separator();
    is_digit_or_letter();
    make_separator();
}

// A function that checks char variables
fn char_vars() {
    let first_char: char = 'A';
    let second_char: char = 'Z';
    let emoji_char: char = '📚';        //  Unicode character
    let non_ascii_char: char = '⾀';   //  Unicode character

    println!(
        "Char variables in Rust: {}, {}, {}, {}",
        first_char, second_char, emoji_char, non_ascii_char
    );
}

// Size of char type variables
fn char_vars_size() {
    let c: char = 'X';
    let emoji_char: char = '📚';
    let non_ascii_char: char = '⾀';
    let german_char: char = 'ß';

    println!(
        "The char type size is bytes is {}.",
        std::mem::size_of::<char>()
    );
    println!(
        "The size of a char {} is {} bytes.",
        c, c.to_string().as_bytes().len()
    );
    println!(
        "The size of {} is {} bytes.",
        german_char, german_char.to_string().as_bytes().len()
    );
    println!(
        "The size of {} is {} bytes.",
        non_ascii_char, non_ascii_char.to_string().as_bytes().len()
    );
    println!(
        "The size of {} is {} bytes.",
        emoji_char,
        emoji_char.to_string().as_bytes().len()
    )
}

// Convert char variables to integer
fn convert_char_vars() {
    let char_value: char = '5';
    let int_value: i32 = char_value.to_digit(10).unwrap_or(0) as i32;

    println!("The conversion of {} to an integer is: {}", char_value, int_value);
}

// Simple function for formatting the output in the console
fn make_separator() {
    println!("{}", "*".repeat(52));
}
