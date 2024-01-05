use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// type alias for a trait object that holds the type of closure that "execute" recieves
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics // good practice for including in docs
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size); // .with_capacity() preallocates space
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers, 
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // explicitly dropping the sender, which causes all calls to .recv() in Worker to return error
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() { // take "takes" the Some variant out and leaves None in its place
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Worker {
        // thread::spawn() will panic if OS can't create a thread, and in turn cause whole server panic, eventhough some threads might succeed
        // In production thread pool imlementation, you'd want to use std::thread::Builder that returns Result instead
        let thread = thread::spawn(move || loop { // this will move the execution to new thread allowing to return a result
            let message = receiver
                .lock()
                .unwrap()
                .recv(); // this blocks the spawned thread if there's no job on receiving end
            match message {
                Ok(job) => {
                    println!(
                        "Worker {id} got a job; executing."
                    );
                    job(); // job is a function
                }
                Err(_) => {
                    println!(
                        "Worker {id} disconnected; shutting down."
                    );
                    break;
                }
            }
        }); 

        Worker { id, thread: Some(thread) }
    }
}

