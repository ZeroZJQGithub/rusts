use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Read Username");
    let username_result = read_username_from_file();
    println!("The user's name is: {:?}", username_result);

    match username_result {
        Ok(name) => println!("The name is: {}", name),
        Err(e) => println!("Problem occure with: {}", e), 
    }

    // let username_result_with_question = read_username_from_file_with_question();
    let username_result_with_question = read_username_from_file_with_question_chains();
    println!("The user's name is: {:?}", username_result_with_question);
    match username_result_with_question {
        Ok(name) => println!("The name is: {}", name),
        Err(e) => println!("Problem occure with: {}", e), 
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    //return Err(e) because the return type is io::Error
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_with_question_chains() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}