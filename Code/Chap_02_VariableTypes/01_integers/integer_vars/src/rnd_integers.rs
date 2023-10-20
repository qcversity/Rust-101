use rand;
// Advanced Section
// ================

pub fn generate_random_signed_integer<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    rand::random()
}
// Print random generated signed integers
pub fn print_rnd_integers() {
    let random_integer_i8: i8 = generate_random_signed_integer();
    let random_integer_i16: i16 = generate_random_signed_integer();
    let random_integer_i32: i32 = generate_random_signed_integer();
    let random_integer_i64: i64 = generate_random_signed_integer();

    println!("Random i8 : {}", random_integer_i8);
    println!("Random i16: {}", random_integer_i16);
    println!("Random i32: {}", random_integer_i32);
    println!("Random i64: {}", random_integer_i64);
}
