use core::panic;
use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub fn collections() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", v);
}

pub fn error_handling() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {:?}", e),
    }

    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found: {:?}", error),
            other_error => {
                panic!("Fatal problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
