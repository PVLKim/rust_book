fn main() {

    // Creating hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // use .insert method
    scores.insert(String::from("Yellow"), 50);
    // all of its keys or values must be of same type!

    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get method returns Option<&V>,
    // .copied returns Option<i32> instead of Option<&i32>
    // .unwrap_or(0) sets score to 0 if scores has no entry for the key

    // Iteration (happens in random order of k/v pairs!)
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Updating

    // overwriting
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // adding only if key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    // .entry returns an enum Entry that represents a value that might/not exist
    // .or_insert method on Entry returns a mut ref to the value iff key exists,
    // if not, it inserts parameter as new value and returns a mut ref to new value

    // updating based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() { // returns iterator over subslices
        let count = map.entry(word).or_insert(0); 
        // map is inserted with key/value and the mut ref to value is stored in count
        *count += 1; // deref count to update value for the key
    }
    println!("{:?}", map);




}
