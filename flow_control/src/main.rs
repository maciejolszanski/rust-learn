fn main() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y!");
    } else if x < y {
        println!("x is smaller than y!");
    } else {
        println!("x is equal to y!");
    }

    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};
    println!("x is {x}");


    let mut count = 0;
    // loop repeats itself forever
    let result = loop {
        if count == 10 {
            break count * 10;
        }

        count += 1;
        println!("count is {count}");
    };
    println!("Loop result is {result}");

    
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    // break in a while loop cannot return a value
    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count +=1;
    }

    let message = ['h', 'e', 'l', 'l', 'o'];

    // the array is converted to an iterator under the hood
    for item in message {
        println!("{item}");
    }

    for number in 0..5 {
        println!("{number}")
    }

    // challenge min, max and average
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mean: f64;
    let mut sum = 0;

    for num in numbers {
        min = if num < min {num} else {min};
        if num > max {
            max = num;
        }
        sum += num;
    }
    mean = sum as f64 / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("test passed!");
}
