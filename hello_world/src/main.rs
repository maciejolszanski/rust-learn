fn main() {

    // Ints and Floats
    let x: u8 = 255;
    println!("x is {}", x);
    // x = x + 1; // will throw an error

    let y1: f32 = 10.123456789123456789;
    let y2: f64 = 10.123456789123456789; //f64 by default
    println!("y1 is {}", y1); // 10.123457 
    println!("y2 is {}", y2); // 10.123456789123457  
    // both values are shorter, and missed 6 -> not equal to initialized value


    let a = 10.0;
    let b = 3.0;
    let c = a / b; // will result in 3 if and and b are i32
    println!("c is {}", c);

    let a = 10;
    let b = 3.0;
    let c = a as f64 / b; // casting to avoid error
    println!("c is {}", c);

    // consider following effect when casting:
    //  3   as f64 -> 3.0
    //  3.9 as i32 -> 3
    //  300 as u8  -> 44
    // -300 as u32 -> 4294966996

    // Formatting Prints
    println!("c is {:.3}", c);  // 3.333
    println!("c is {:8.3}", c); // "   3.333" <- spaces so the result has 8 total characters
    println!("c is {:08.3}", c); // 0003.333 <- zeros instead of spaces
    println!("c is {:08.3}\na is {}", c, a); 
    println!("c is {1} a is {0:.3}", c, a); // "c is 10 a is 3.333" <- inverted order of parameters
    println!("c is {number:.3}", number=c); 
    println!("c is {c:.3}"); //captures variable c from local scope

}
