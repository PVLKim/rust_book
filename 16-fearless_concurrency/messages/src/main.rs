use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Message passing concurrency allows threads to pass data between each other
    // It uses channel that has two halves: transmitter(tx) and receiver(rx)

    // mpsc - multiple producer single consumer (multiple streams flowing into one big river)
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // here we clone transmitter to allow multiple producers
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val)
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val)
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receiver has two useful methods .recv() and .try_recv()
    // .recv() blocks main thread execution and waits for result. When transmitter closes, it will return an error to signal that no more values are coming
    // .try_recv() doesn't block, but will return Result<T,E> immediately, useful for better parallelism

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {received}");
    }
;


}
