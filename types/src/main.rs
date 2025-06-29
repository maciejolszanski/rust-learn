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


    // bitwise operations
    let mut value = 0b1111_0101u8;
    println!("value is {value}"); // 245
    println!("value is {value:08b}"); // 11110101

    value = !value;
    println!("value is {value:08b}"); // 00001010

    // AND is used to clear one bit where the zero is placed
    // or to check if specified bit is 1 or 0
    value = value & 0b1111_0111; 
    println!("value is {value:08b}"); // 00000010
    println!("bit 6 is {}", value & 0b0100_0000); // 0

    // OR is used to set specific bit to 1
    value = value | 0b0100_0000; 
    println!("value is {value:08b}"); // 01000010

    // XOR to check where the values differ between two bit patterns
    value = value ^ 0b0101_0101;
    println!("value is {value:08b}"); // 00010111

    // Bit shifting
    value = value << 4;
    println!("value is {value:08b}"); // 0111000
    value = value >> 2;
    println!("value is {value:08b}"); // 0001110


    // Boolean
    /*  
    We can short-circuit logical operations
    false & <anything> -> false
    true | <anything>  -> true

    so when we use && or || the right sight will not even be evaluated if the left side is 
    false for && 
    true for ||
    */
    let c = (false & true) && panic!(); // will not panic
    println!("c is {c}");
    //let c = (false & true) || panic!(); // will panic


    // Char
    let letter = 'a';
    let number = '1';
    let finger =  '\u{261D}'; // using hexadecimal unicode value
    println!("{letter}\n{number}\n{finger}");

    // Arrays
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("{first_letter}");

    let numbers: [i32; 5]; // empty array
    numbers = [0; 5]; // array of 5 copies of a value 0
    let index: usize = numbers.len(); // usize is based on a number of bytes needed to reference memory
    //println!("Last number is {}", numbers[index]); // will compile but panic during runtime

    // Multi-dimensional arrays
    let parking_lot = [[1,2,3],
                                      [4,5,6]];
    let number = parking_lot[0][1];
    println!("{number}");

    // inner arrays must be the same size - if not they are treated like different types
    // let parking_lot = [[1,2,3],
    //                    [4,5,6,7]]; // will panick

    let garage: [[[i32; 100]; 20]; 5];

    // Tuples
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first tuple item is {first_item}");

    let (a, b, c) = stuff;
    println!("b is {b}")
}
