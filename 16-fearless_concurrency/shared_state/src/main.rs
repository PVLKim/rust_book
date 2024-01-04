use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared state concurrency implies several threads have access to the same memory location
    // To avoid race conditions, Rust introduced mutexes (mutual exclusion).
    // Mutex allows only one thread to access some data at any given time. 
    // To access data in mutex, a thread must signal by acquiring a mutex's lock.
    // ! Mutex<T> comes with the risk of creating deadlocks 
    // ex: operation needs to lock 2 resources and 2 threads have each acquired one of the locks, causing them to wait for each other)
    
    let m = Mutex::new(5); // declared as immutable, but provides interior mutability
    println!("m = {:?}", m);
    {
        let mut num = m // declaring as mutable to be able to modify internal value
            .lock()
            .unwrap();  // acquiring the lock and if busy, it will panic
                        // returns a smart pointer MutexGuard which impl Deref and Drop

        *num = 6; // accessing the value inside mutex
    }
    println!("m = {:?}", m);

    // Sharing mutex between multiple threads
    // atomic reference counter (thread-safe, but comes with perf overhead)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles { // this exists to make sure all threads finish computing before returning the final result
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
