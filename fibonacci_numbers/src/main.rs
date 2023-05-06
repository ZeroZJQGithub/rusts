use std::io;

fn main() {
    println!("Fibonacci Numbers");

    let mut fibonacci: [u64; 101] = [0; 101];
    println!("array is {:?}", fibonacci);

    fibonacci[0] = 1;
    fibonacci[1] = 1;
    println!("array is {:?}", fibonacci);

    let mut max_number = String::new();
    io::stdin().read_line(&mut max_number).expect("Failed to read line.");
    let max_number: usize = max_number.trim().parse().expect("Input a number!");

    if max_number <= 90 {
        for index in 2..=max_number {
            fibonacci[index ] = fibonacci[index -1] + fibonacci[index -2];
        }
        for element in fibonacci.iter() {
            println!("the value is {element}");
        }
    } else {
        println!("Max number can not over 90!");
    }

}
