fn main() {

    let v: Vec<i32> = Vec::new(); // create new vector
    let v = vec![1, 2, 3]; // same with macro, with type inference

    // Updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // Reading elements of Vector
    let v = vec![1, 2, 3, 4, 5];

    // via indexing
    let third: &i32 = &v[2]; // extracting a ref from v
    println!("The third element is {third}");
    // via method 'get'
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // won't compile, since immutable borrow happens above
    // 

    // Iterating over the values of vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    // to modify the vector, need to do a mutable borrow
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // !need derefencing operator (*)
    }
    println!("Modified vector: {:?}", v);


    // To store multiple types in vector, use enums
    // Enum defines the "schema" of the vector
    // But this requires to know in advance all types
    // If you don't know, you need to use a trait object (see later)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


}
