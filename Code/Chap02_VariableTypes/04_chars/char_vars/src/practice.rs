// Example: Check if a char is a vowel
pub fn is_vowel() {
    let characters = ['E', 'H', 'i', 'o', 'U', 'Y'];

    for &character in &characters {
        match character {
            'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => {
                println!("{} is a vowel.", character);
            }
            _ => {
                println!("{} is not a vowel.", character);
            }
        }
    }
}


// Convert a char to its ASCII value
pub fn char_to_ascii() {
    let characters = ['A', 'a', 'R', 'u', 'S', 'T', 'P'];

    for &char_value in &characters {
        let ascii_value: u8 = char_value as u8;
        println!("The ASCII value of {} is: {}", char_value, ascii_value);
    }
}

// Check whether a char is a digit or a letter
pub fn is_digit_or_letter() {
    let characters = ['7', 'A', '0', '⍵'];
    for &char_value in &characters {
        if char_value.is_digit(10) {
            println!("{} is a digit.", char_value);
        } else if char_value.is_alphabetic() {
            println!("{} is a letter.", char_value);
        } else {
            println!("{} is neither a digit nor a letter.", char_value);
        }
    }
}
