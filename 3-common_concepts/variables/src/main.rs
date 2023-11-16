fn main() {

    // mutablle variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // can reassign variables without `let`
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //  

    // shadowing
    let x = 5;
    let x = x + 1; // shadowing allows to reassign the variable and even type!
    
    // once the inner scope ends, `x` becomes 6 again!
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", 5 / 3); // floor division
    println!("{}", 5.0 / 3.0); // 

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Values: {c}, {z}, {heart_eyed_cat}");

    // compound types
    // tuples
    let tup = (500, 6.4, 1); // can take multiple types
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    println!("The value of y is: {y}");
    println!("First value of tuple: {five_hundred}")

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; 
    let a = [3; 5]; // ~ a = [3,3,3,3,3]
    let first = a[0];

}