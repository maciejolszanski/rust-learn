fn main() {
    let x = 1;
    let y = 2;
    say_a_sum(x, y);
    say_a_number(x as i32); // will fail if not casted, becuase the x and y are created as u8 (that's how they are used the first time)

    let result = square_v2(13);
    println!("result is {result:?}");

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

//rust doesn't care about the order of defining functions
fn say_a_number(number: i32) {
    println!("number is {number}");
}

fn say_a_sum(a: u8, b:u8) {
    println!("sum is {}", a + b);
}

fn square(x: i32) -> i32 {
    println!("squaring {x}");
    x * x // it's an expression because doesn't have a semicolon - it'll be returned
}

//but we can also do it using return and adding semicolon -> changin it to a statement
fn square_v2(x: i32) -> (i32, i32) {
    println!("squaring {x}");
    return (x, x * x); // it's an expression because doesn't have a semicolon - it'll be returned
}

fn celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    1.8 * celsius_temp + 32.0
}
