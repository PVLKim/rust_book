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

    

}
