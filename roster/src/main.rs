use std::fs;
use std::env;

fn main() {
    let cl_args: Vec<String> = env::args().collect();
    if cl_args.len() <= 2 {
        println!("You must provide two arguments: path to file, and name to search for.");
        return;
    }

    let path = &cl_args[1];
    let name_to_search = &cl_args[2];

    let names = fs::read_to_string(path).unwrap();
    
    if names.contains(name_to_search) {
        println!("Name exists in the file!")
    }
    else {
        println!("Name doesn't exist in the file!")
    }
    
}
