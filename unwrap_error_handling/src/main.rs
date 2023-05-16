use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Handling error with unwrap_or_else");

    let data_file = File::open("data.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("data.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("The data file is: {:?}", data_file);
}
