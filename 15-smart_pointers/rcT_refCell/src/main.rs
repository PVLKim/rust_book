// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

pub mod refcycles;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    // Via immutable refereces Rc<T> allows you to share data between multiple parts 
    // of your program for reading only.
    // Below is an example that shows how ref count changes

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!(
    //     "count after creating a = {}",
    //     Rc::strong_count(&a),
    // );
    // let b = Cons(3, Rc::clone(&a)); // Rc::clone() increments reference count
    // println!(
    //     "count after creating b = {}",
    //     Rc::strong_count(&a),
    // );
    // {
    //     let c = Cons(4, Rc::clone(&a));
    // println!(
    //     "count after creating c = {}",
    //     Rc::strong_count(&a),
    // );
    // }   
    // println!(
    //     "count after c goes out of scope = {}",
    //     Rc::strong_count(&a),
    // );

    /*
    A common way to use RefCell<T> is in combination with Rc<T>. 
    Rc<RefCell<T>> allows to get a value that can have multiple owners and that you can mutate.
    This works only in single-threaded scenarios, for thread-safe version use Mutex<T> instead.
    */

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); // Rc::clone(&value) allows both 'a' and 'value' maintain ownership over inner value 5
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    refcycles::main();
    
}