use crate::List::{Cons, Nil};
use std::ops::Deref;


enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Implementing Drop trait and its drop method
// lets you customize what happens when a value is going out of scope

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // this prints to console when the struct variable goes out of scope
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Way to define Cons list (Recursive DS)
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref operator points to real value
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // impl Deref trait allow deref coercion (&MyBox<String> -> &String)
    // String impl Deref trait, which in turns allows converting to &str
    // This allows to use custom Box types without explicit conversion to string slice as defined in func signature
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Sometimes you might want to drop value early, so for that you can use 
    // std::mem::drop function 
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomerSmartPointer created");
    std::mem::drop(c);
    println!("CustomerSmartPointer dropped before the end of main");





    

}
