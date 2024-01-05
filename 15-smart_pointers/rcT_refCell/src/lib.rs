/*
Interior mutability is a design pattern in Rust that allows you to mutate data even when 
there are (!) immutable references to that data (normally it's not allowed by borrowing rules).
To mutate data, this pattern uses 'unsafe' code inside a data structure to bend Rust's 
usual rules that govern mutation and borrowing. Unsafe code indicates the compiler that 
we're checking the rules manually instead of relying on the compiler to check them for us.
Unsafe code is wrapped in a safe API, and the outer type is still immutable.
    */ 

// RefCell<T> type is the type that follows the pattern and represents single ownership 
// over the data it holds. It's used only in single-threaded scenarios. Because it allows
// mutliple borrows checked at runtime, you can mutate the value inside the RefCell<T>
// even when the RefCell<T> is immutable (! essence of interior mutability pattern)

// RefCell<T> use case for interior mutability pattern: mock objects
// This shows the case when it's useful for a value to mutate itself in its methods but
// appear immutable to other code. 



// this is the interface that our mock objects need to implement
pub trait Messenger {
    // takes immutablke ref to self and message text
    fn send(&self, msg: &str); 
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where 
    T: Messenger,
{
    pub fn new(
        messenger: &'a T,
        max: usize
    ) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // this tests the 
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = 
            self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger
                .send("Error: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent: You're at 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You're at 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut() // this allows to get a mutable ref to the value inside RefCell
                .push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100,
        );

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // .borrow() gives an immutable ref
    }

    /*
    .borrow_mut() and .borrow() return the smart pointer types RefMut<T> and Ref<T> respectively,
    which both impl Deref and therefore, can be used as regular references.

    RefCell keeps track of these smart pointers and every time we call borrow(), 
    RefCell<T> increases its count of how many immutable borrows are active.
     */
}