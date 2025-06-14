fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_char = 0;
    let mut last_char = 0;
    let mut look_for_leading = true;
    let mut look_for_trailing = false;

    for (index, &byte) in bytes.iter().enumerate() {
        if byte != b' ' && look_for_leading {
            first_char = index;
            look_for_trailing = true;
            look_for_leading = false;
        } else if byte != b' ' && look_for_trailing {
            last_char = index + 1;
        }
    }

    &s[first_char..last_char]
}