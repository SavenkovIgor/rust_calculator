use std::io;

fn main() {
    println!("Print the expression to calc. Using only 0-9+-/*^");
    let mut expression = String::new();
    io::stdin()
    .read_line(&mut expression)
    .expect("Failed to read input");
    
    println!("Entered string is {}", expression);
}
