fn main() {
    // Send marker trait allows transfering ownership of the type implementing it between threads
    // Most types are Send, except Rc<T> and raw pointers

    // Sync marker trait allows accessing the type implementing it from multiple threads
    // in other words, any type T is Sync if &T is Send
    // Most types are Sync except Rc<T>, RefCell<T> and Cell<T> types

    // ! Any types that are made of Send and Sync are automatically Send and Sync (e.g structs/enums composed of primitive types)
    // as marker traits, they have no method to implement

}
