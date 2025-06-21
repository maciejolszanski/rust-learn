use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message!");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {buffer}");

    // Parsing input
    // parse::<i32> is a turbofish operator
    let number_1 = buffer.trim().parse::<i32>().unwrap();
    let number_2: i32 = buffer.trim().parse().unwrap();
    println!("numer_1 + 1 is {}", number_1 + 1);
    println!("numer_2 + 1 is {}", number_2 + 1);
}
