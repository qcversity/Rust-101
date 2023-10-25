fn main(){
    make_separator();
    print_statements();
    make_separator()
}

// Understanding the println!() 

fn print_statements(){
    println!("A simple message.");
    println!("The println statement can use curly braces `{}{}` as a placeholder.", '{', '}');
    println!("This is a char value: {}", 'A');
    println!("Multiple placeholders: {}, {}, {}, {}.", 102, 'B', true, "Message")
}

fn make_separator(){
    println!("{}", "*".repeat(62))
}