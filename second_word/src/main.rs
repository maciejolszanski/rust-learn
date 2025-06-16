fn main() {
    let test1 = "We need more space.";
    assert_eq!(get_second_word(test1), "need");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(get_second_word(&test2), "space");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(get_second_word(&test3[..]), "space");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(get_second_word(test4), "surrounded");
    
    let test5 = "     ";
    assert_eq!(get_second_word(test5), "");
    
    let test6 = "";
    assert_eq!(get_second_word(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(get_second_word(test7), "");
    println!("Tests passed!");
}

fn get_second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start_index = 0;
    let mut end_index = 0;
    let mut looking_for_first_word = true;
    let mut looking_for_second_word = false;
    let mut reading_second_word = false;

    for (index, &byte) in bytes.iter().enumerate() {
        if looking_for_first_word && byte != b' ' {
            looking_for_second_word = true;
            looking_for_first_word = false;
        } 
        else if looking_for_second_word && byte == b' ' {
            start_index = index + 1;
            reading_second_word = true;
            looking_for_second_word = false;
        } 
        else if reading_second_word && byte == b' ' {
            end_index = index;
            break
        }
    }
    if end_index < start_index {
        end_index = start_index
    }

    &s[start_index..end_index]
}