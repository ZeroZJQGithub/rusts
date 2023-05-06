use std::io;

fn main() {
    println!("Hello, world!");
    println!("Input guess number:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess number is {guess}");
}
