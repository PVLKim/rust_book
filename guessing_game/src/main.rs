use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Guess the number!"); // !println is a macro, println is a function

    let secret_number = rand::thread_rng().gen_range(1..=100);
   
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // :: -> new is an associated function of String type

        io::stdin()
            .read_line(&mut guess) // `read_line` appends the input to argument string. Hence mutable argument is needed
                                   // & indicates that guess is a reference. 
                                   // References are by default immutable, so `mut` is needed 
            .expect("Failed to read line"); // .read_line also returns "Result" enum which has 2 variants (Ok/Err)
    
        // shadowing allows us to reuse the existing mutable variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}"); 
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}