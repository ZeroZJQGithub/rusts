use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Error Handling!");

    let hello_file_result = File::open("data1.txt");

    let hello_file = match hello_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("data1.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            orther_error => panic!("Problem opening the file: {:?}", orther_error),
        }
    };

    println!("this file is: {:?}", hello_file);
}
