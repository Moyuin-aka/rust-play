
use std::io;
fn main() {
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read number");

    let number:i32 = input.trim().parse().expect("Failed to read number");
    if number % 3 == 0 {
        println!("the number {} is OK", number);
    } else if number % 2 == 0 {
        println!("the number {} is OK", number);
    } else {
        println!("the number {} is No", number);
    }
}
