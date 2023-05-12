fn main() {
    println!("Hello, world!");

    let some_number = Some(5);
    let some_string = Some("Hello");
    let none_type: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", none_type);
}
