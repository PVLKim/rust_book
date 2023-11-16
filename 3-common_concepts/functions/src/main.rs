fn main() {
    println!("Hello, world!");
    another_function();
    let x = five();
    println!("The value of x is: {x}");
    func();
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5
}

fn func() {
    let y = {
        let x = 3;
        x + 1 // no semicolon, since it's expression
    };

    println!("The value of y is: {y}");
}