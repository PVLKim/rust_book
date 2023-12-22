use std::backtrace;

fn main() {

    // Matching named variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // since match x creates a new scope 
                                                      // 5 will bind to y and match this arm
        _ => println!("Default case, x = {:?}", x),
    }

    // Matching ranges
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a' ..='j' => println!("early ASCII letter"),
        'k' ..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring to create variables from struct fields
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // or as a shorthand
    let Point {x, y} = p;
    println!("{x} and {y}");

    // destructuring according to pattern
    match p {
        Point { x, y: 0} => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!(
                "The Quit variant has not data to destructure"
            );
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x dir {x}, in the y dir {y}"
            );
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r,g,b ) => println!(
            "Change color to red {r}, green {g}, blue {b}"
        ),
    }


    // Destructuring nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum NewMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        NewMessage::ChangeColor(Color::Rgb(r,g ,b )) => println!(
            "Change color to red {r}, green {g}, blue {b}"
        ),
        NewMessage::ChangeColor(Color::Hsv(h,s ,v )) => println!(
            "Change color to hue {h}, saturation {s}, value {v}"
        ),
        _ => (),
    }

    // Destructuring structs and tuples
    let ((feet, inches), Point {x, y}) =
        ((3, 10), Point {x: 3, y: -10 });
    

    // Ignoring values in the pattern

    // entire value:
    fn foo(_: i32, y: i32) {};

    // parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // to test if both values are the Some variant
        println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // to avoid warning about unused variables you can prefix the name with underscore
    let _x = 5;
    // still _ creates a variable 
    let s: Option<String> = Some(String::from("Hello!"));
    if let Some(_s) = s { // this binds s to _s and moves the value to it
        println!("found a string");
    }
    let s: Option<String> = Some(String::from("Hello!"));
    if let Some(_) = s { // this won't bind s
        println!("found a string");
    }
    println!("{:?}", s); // this wouldn't compile for _s case

    // Remaining parts
    struct Point3D {
        x: i32,
        y: i32,
        z: i32
    }
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    }
    // or
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    // but never {.., second, .. }, because it'd be ambiguous

    // Match guards allow specifying additional condition
    let x = Some(5);
    let y = 10;
    match x {
        Some(n) if n == y => println!("Matched, n = {n}"), // no binding here unlike in first example
        _ => println!("Default case, x = {:?}", x),
    }

    // to specify multiple patterns:
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // Bindings let us create variable that holds a value at the same time 
    // we are testing that value for a pattern match
    enum Message1 {
        Hello { id: i32 },
    }
    let msg = Message1::Hello { id: 5 };
    match msg {
        Message1::Hello {
            id: id_variable @ 3..=7, // assigns value to id_variable
        } => println!("Found an id in range: {id_variable}"),
        Message1::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message1::Hello { id } => println!("Some other id: {id}"),
    }
}