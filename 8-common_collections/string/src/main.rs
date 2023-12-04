fn main() {

    // Methods
    let s = "initial contents".to_string(); // available for types that implement 
                                            // Display trait
    let mut s = String::from("foo");
    s.push_str("bar"); // append string slice to String 
    s.push('s'); // appends char 

    // Concatenating strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // '+' operator is implemented with 'add' method, which looks like this:
    // fn add(self, s: &str) -> String { .. } 
    // compiler can take the second parameter as String, 
    // and apply a deref coercion (&s2 -> &s2[..]
    // self doesn't have &, so 'add' takes ownership of self

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3; // possible, but looks bad
    let s = format!("{s1}-{s2}-{s3}");     // neat way to combine strings
                                           // format! takes refs 


    // Indexing doesn't exist for strings 
    let hello = String::from("Hola"); // 4 bytes
    let hello = String::from("Здравствуйте"); // 24 bytes
    // let answer = &hello[0]; // won't compile

    
    // String slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}"); // prints 'зд'

    // to avoid confusion as above, be explicit if u want characters or bytes
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }


}
