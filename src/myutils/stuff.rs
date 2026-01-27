use core::panic;
use std::{
    fmt::Display,
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

pub fn lifetimes() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("lifetimes: the longest string is {}", result);

        longest_with_announcement(string1.as_str(), string2.as_str(), "hoho!");
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
