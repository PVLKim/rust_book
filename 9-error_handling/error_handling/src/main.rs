use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // for panicking, use macro
    // panic!("crash and burn");

    // Matching on error
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };


    // alternative way, using closures and `unwrap_or_else` method
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // .unwrap method on Result<T,E> which returns the value inside Ok() or call the panic!
    let greeting_file = File::open("hello.txt").unwrap();

    // .expect in case of error, adds the message to panic! , preferrable over .wrap
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");


    // Propagating the error to the caller of the function instead of panicking
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
    
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e), // for early return, specify return explicitly
        };
    
        let mut username = String::new();
    
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }

    // Shortcut for propagating errors -> ?
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    
}