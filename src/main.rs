use std::io;

fn sym_check(check_string: &String) -> bool {
    static ACCEPTABLE_SYMBOLS: &str = "0123456789.+-/*^";

    for sym in check_string.chars() {
        if !ACCEPTABLE_SYMBOLS.contains(sym) {
            return false
        }
    }

    return true
}

fn read_input() -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read input");
    return ret.replace("\n", "");
}

fn main() {
    println!("Print the expression to calc. Using only 0-9.+-/*^");
    let expression = read_input();

    if sym_check(&expression) {
        println!("Entered string {} is valid", expression);
    } else {
        println!("Entered string {} is invalid", expression);
    }    
}
