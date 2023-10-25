use std::any::Any;

fn main() {
    make_separator();
    infer_var_type();

    make_separator();
    print_var_type();

    make_separator();
    unused_vars();

    make_separator();
    cast_vars();

    make_separator();
    redeclare_vars();

    make_separator()
}

fn infer_var_type() {

    // Rust lang can infer var types, so we don't need always to explicitly declare the var type.
    let a = -12700;
    let b = 2.71828;
    let c = 'X';
    println!("a is {}, b is {}, c is {}", a, b, c);
}

// Define a function that checks the variable type during runtime.

fn check_type(value: &dyn Any) {
    if value.is::<i32>() {
        println!("The value is of type i32");
    } else if value.is::<bool>() {
        println!("The value is of type bool");
    } else if value.is::<char>() {
        println!("The value is of type char");
    } else if value.is::<f64>() {
        println!("The value is of type f64");
    } else {
        println!("The value is of an unknown type");
    }
}

// Use the previous function to check the type of variables. 

fn print_var_type(){
    let x = 32;
    let y = true;
    let z = 'A';
    let t = -234;
    let f = 2.71828;

    check_type(&x);
    check_type(&y);
    check_type(&z);
    check_type(&t);
    check_type(&f);
}

//  Unused variables 
// --------------------
// If you don't use a variable, or you intend to use it in a later stage
// you can prefix name with _ to avoid compiler warning.

fn unused_vars () {
    let _f = 0;
    let _is_ok = true;
    println!("There some variables defined in this func, however, the compiler \ndoes not complain about not using them.")
}


// Casting
fn cast_vars(){
    // You can cast using the "as" keyword.
    let g = 3.99;
    let h = g as i32;
    println!("g is {}, h is {}", g, h);
}

// Redeclaring

fn redeclare_vars (){
    // Rust enables you to redeclare a variable in the current scope. This is called shadowing and it's quite cool.
    let num = String::from("1270101");
    println!("num {} is of type: `{}` with size of `{}` bytes.", num, std::any::type_name::<String>(), num.to_string().as_bytes().len());
    let num = 1270101;
    println!("num {} is of type: `{}` now with size of `{}` bytes", num , std::any::type_name::<i32>(), std::mem::size_of_val(&num));
}

// Simple function for formatting the output in the console
fn make_separator() {
    println!("{}", "*".repeat(72));
}