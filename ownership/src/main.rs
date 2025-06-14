fn main() {
    // shadowing and scoping
    let planet = "Earth";
    {
        println!("Planet is {planet}"); // Earth
    
        let planet = 4;
        println!("Planet is {planet}"); // Mars
    }
    println!("Planet is {planet}"); // Earth


    /* Stack and Heap

    Program Memory in Rust is divided in to Stack and heap
    Stack (like a pile of boxes)
     - values are stored in sequential order
     - data addes and removes as last in, first out (LIFO)

     + Data can be pushed and popped very quickly
     - small size
     - all data must have a known, fixed size (every bok in a stack has a size which cannot be changed)
    
    Heap (storing boxes of data in a giant warehouse)
     - as part of allocation, when rust assigns us a chunk of memory it gives us the location of that memoery,
       so we can access it using a pointer
    Pointer 
     - is a data type that stores a memory address
     - is a fixed size, so can be storead on a stack
     We can fixed of pointers as an inventory sheet which tells us where to find a specific box in the warehouse

     + We can dynamically add and remove data in a Heap
     + big size
     - Accessing data in a Heap is slower (we have to follow a pointer)
     - Adding data is also slower, as we have to look for available space
     
    */

    /* String
    Int, Float, Bool, Char, Array and Tuple - all of them have a fixed size known to compiler - can be stored in a Stack
    String - is stored on the Heap

    String literals like "hello"
     - are hard-coded by compiler into .exe
     - are immutable
     - value must be known before compilation
    
    For other use cases, like user input we'll have to use String Type
    
    String Type:
     - allocated on the heap
     - mutable
     - dynamically generated at runtime
    */

    let mut message = String::from("Earth");
    println!("message is {message}");
    message.push_str(" is home");
    println!("message is {message}");

    /*
    Heap is big, but not infinite, so we have to clean up allocated memory that is no longer needed
    This can be managed by programmed in C/C++, or by garbage collector in Java, Python, C#, Ruby, Go

    Rust is using ownership for that, so variables are responsible for freeing their own resources according to the set of rules,
    that can be checked by the compiler:
      1. Every value is "owned" by one, and only one, variable at a time
      2. When the owning variable goes out of scope, the value is dropped

    Advantages:
      - safe - it protects from the bugs like "invalid access"
      - efficient - compiler can establish when the variable will be needed, no garbage collector running in the background

    Disadvantages:
      - requires understanding of ownership

    */

    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {inner_planet}");
        // new variable in a stack is pointing to the same heap memory (sth like shallow clone),
        // but the previous pointer is invalidated, the ownership was moved
        outer_planet = inner_planet;
        // println!("inner_planet is {inner_planet}");  // will fail here
        // if we want to copy the variable, we can use .clone() (like deep clone) - it'll duplicate data in heap, operation on one variable will not affect the other
    }
    println!("outer_planet is {outer_planet}");

    // But Rust behaves differently for variables that are stored in Stack
    // Data types with a known size, will implement the trait that allows them to be copies rather than moved like Strings
    // Copying occuers implicitly, cloninig must be done explicitly
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        println!("inner_planet is {inner_planet}"); //1
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {inner_planet}"); // 2
    }
    println!("outer_planet is {outer_planet}"); // 1

    // Transfering ownership

    // since the rocket_fuel is kept in Stack, when passing it as an argument the value is copied, 
    // so there's new variable propellant in the Stack
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");

    /* 
    When passing string as argument, the propellant is a new pointer to the same heap memory
    so it takes ownership of it, but that means we are transfering ownerhip from rocket_fuel variable to the propellent variable
    when the function is exited, the memory is disallocated, because the propellent variable is no longer needed
    rocket_fuel variable cannot be used, because it no longer ownes the String
    */
    let rocket_fuel = String::from("RP-1");
    process_fuel_2(rocket_fuel);
    // println!("rocket_fuel is {rocket_fuel}"); // will fail


    // first solution is to .clone() the variable
    let rocket_fuel = String::from("RP-1");
    process_fuel_2(rocket_fuel.clone());
    println!("rocket_fuel is {rocket_fuel}");

    // second solution is to pass the ownership back to the rocket_fuel variable
    // the rocket_fuel returned by function doesn't have to have the same value
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel_3(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");
}

fn process_fuel(mut propellant: i32) {
    propellant += 1;
    println!("rocessing propellant {propellant}...");
}

fn process_fuel_2(propellant: String) {
    println!("rocessing propellant {propellant}...");
}

fn process_fuel_3(propellant: String) -> String {
    println!("rocessing propellant {propellant}...");
    let new_fuel = String::from("LNG");
    new_fuel
}
