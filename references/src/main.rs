fn main() {
    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel, length) = process_fuel(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");

    /* Borrowing references/ownership
    It enables us to access data without taking ownership of it (or borrowing the ownership)
    It's done with '&' operator.

    In the function definiition the paramter has to have & and the passed argument, too.

    The propellant variable in Stakc will be pointing to the rocket_fuel variable in stack that is pointing to data in Heap

    */
    let rocket_fuel = String::from("RP-1");
    let length = process_fuel_borrow(&rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");

    // If we want to change the borrowed variable we have to explicitly tell Rust we want to do it by making the original variable mutable
    // as well as specifying it in both the function definition and call as a mutale reference (&mut rocket_fuel)
    // When using a mutable reference you cannot create any other references to it within that scope
    // it prevents data races
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel_modify(&mut rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");


    // Dangling reference
    // The string created in produce_fuel function is dropped on exit of the function
    // so the reference points to a memeory that can be already allocated to something else
    // 
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel is {rocket_fuel} and length is {length}");

    /* 
    Slice refernces data - doesn't take ownership of it
     - commonly encountered as string slice data type &str
     - string literals are slices

    message is still an owner of the whole String
    the slice has a pointer to the letter E, and the length of the slice
    */
    let message = String::from("Greetings from Earth!");
    println!("message is {message}");

    let last_word = &message[15..15+5];
    println!("last_word is {last_word}");

    // it compiles becuase String change its length uring runtime, but it'll panic in the runtime as it wasn't changed in this case
    // let last_word = &message[15..15+50]; 
    // println!("last_word is {last_word}");

    // slice to the end
    let last_word = &message[15..]; 
    println!("last_word is {last_word}");

    // Length of slice is in bytes (not characters), so if the character has more than one byte, the slice can be different that what we wanted
    // It can happen for special characters or emojis

    // slice of arrays
    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets); //note the :?


    // Slice as function parameters
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {first_word}");

    /*
    rememeber that &String is different than &str
    The firs one (reference) will poin to the actual variable in a Stack, that owns the Heap data (pointer to the beggining, length, capacity)
    The slice stores a pointer to the Heap data along the length information (only pointer and length)
    Rust will allow us to use &String in place of &str (reference in place of slice) as it has all needed information (an more, becuase it has also capacity)
    it's called Deref coercion, will use &String as a slice which length is the entire String
    but the coerction doesn't work the other way, as slice doesn't have to capacity information
    */
    
    // let first_word = get_first_word(&message[10..]); // it will fail
    // println!("first_word is {first_word}");
    let first_word = get_first_word_2(&message[10..]);
    println!("first_word is {first_word}");
    let first_word = get_first_word_2(&message);
    println!("first_word is {first_word}");
    // We should use &str as input and output, because it gives us more flexibility

}

fn process_fuel(propellant: String) -> (String, usize) {
    println!("rocessing propellant {propellant}...");
    let length = propellant.len();
    (propellant, length)
}

fn process_fuel_borrow(propellant: &String) -> usize {
    println!("rocessing propellant {propellant}...");
    let length = propellant.len();
    length
}

fn process_fuel_modify(propellant: &mut String) -> usize {
    println!("rocessing propellant {propellant}...");
    propellant.push_str(" is highly flammable!"); // it will modify the original variable that is passed to the function (rocket fuel, not propellant)
    let length = propellant.len();
    length
}

// It's better to return the new_fuel String, so the ownership will be moved
// fn produce_fuel() -> &String {
//     let new_fuel = String::from("RP-1");
//     &new_fuel
// }

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s // uf no spaces found -> input is a single word
}

fn get_first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s // uf no spaces found -> input is a single word
}