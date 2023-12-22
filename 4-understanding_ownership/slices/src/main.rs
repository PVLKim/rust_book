fn main() {

    // having string slice as return type doesn't allow mutating the original variable s
    // below will produce error, since `word` is still being used
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); 
    // println!("the first word is: {}", word); // commented to avoid error


    // Because &str is being used, all calls below are valid
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]); // ref to slice of String
    let word = first_word(&my_string[..]); 
    let word = first_word(&my_string);       // ref to String

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]); // slice of string literal
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);        // string literal 
                                                     // (they are already string slices!)


}

// parameter type `s: &str` instead of `s: &String` allows
// to pass string slices directly or slice of the String or reference to the String
fn first_word(s: &str) -> &str { // `&str` signifies string slice type
    let bytes = s.as_bytes(); // for elem-by-elem iteration, we need to convert s to bytes

    // .iter() creates iterator, &item is a reference of element
    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // reference of entire s
}
