/*
Example of ,emory leaks with reference cycles
*/



#[derive(Debug)]
enum RefList {
    Cons(i32, RefCell<Rc<RefList>>), // we want to be able to modify the List value
    Nil,
}

impl RefList {
    fn tail(&self) -> Option<&RefCell<Rc<RefList>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use RefList::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}