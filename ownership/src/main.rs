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

}
