use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {

    // Command line arguments
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument is {index} is {argument}");
    }

    // We can use .nth() to get the nth argument from the iterator
    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {arg2}");

    // Reading from files
    let content = fs::read_to_string("planets.txt").unwrap();
    println!("content is {content}");

    for line in content.lines() {
        println!("line is {line}");
    }

    let content = fs::read("planets.txt").unwrap();
    println!("content is {:?}", content);

    // Writing to file
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon is this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);
    // Write function overwrites the file and writes the entire content at once

    // If we want to update the content we have to do it as follows:
    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");
    // It ahs to be interpetered as bytes values so we have to add a b prefix
} 
