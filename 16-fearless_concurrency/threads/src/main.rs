use std::thread;
use std::time::Duration;

fn main() {

    // When main thread finishes, all spawn thread are shut down (6..10 won't be printed)
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1)); // pauses execution, allowing other threads to run
    //     }
    // });
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // To avoid premature finish of spawned threads:
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // this will block the main thread and wait until the spawned thread is finished
                            // if this is put before the second for-loop, the main thread will block, so there will be no parallelism  


    // Now, let's try to use closures that capture a vector
    // Rust will try to infer that closure only needs to borrow reference to v, but
    // would still not work without move statement, since the new thread might outlive current scope
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || { // this will transfer ownership of v to the spawned thread
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
  
}
