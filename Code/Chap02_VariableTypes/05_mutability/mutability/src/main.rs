fn main() {
    make_separator();
    immutable_vars();
    
    make_separator();
    mutable_vars();

    make_separator();
    compute_sum_for();

    make_separator(); 
    compute_product_while();

    make_separator()
}

// Immutable variables
// Variables are immutable by default.

fn immutable_vars(){
    
    let imm_var = 0;
    //imm_var = 1;
    println!("The immutable variable is {}", imm_var);
}

// Mutable variables
// You must use the mut qualifier to make a variable mutable.

fn mutable_vars (){ 
    let mut var = 0;
    println!("e originally is {}", var);
    var = 1;
    println!("e afterwards is {}", var);
}

// Mutable variables are used in loops such as `for` or `while`
fn compute_sum_for()  {
    let mut total = 0;
    let values = vec![1, 2, 3, 4, 5];

    for x in values {
        total += x;
    }
    println!("Sum of values: {}", total);
}

fn compute_product_while() {
    let mut total = 1;
    let values = vec![1, 2, 3, 4, 5];
    let mut iter = values.iter();

    while let Some(x) = iter.next() {
        total *= *x;
    }

    println!("Product of values: {}", total);
}


// Simple function for formatting the output in the console
fn make_separator() {
    println!("{}", "*".repeat(52));
}